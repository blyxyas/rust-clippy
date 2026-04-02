use clippy_config::Conf;
use clippy_config::types::MacroMatcher;
use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::{SourceText, SpanRangeExt, snippet};
use rustc_ast::ast;
use rustc_ast::tokenstream::TokenStream;
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_errors::Applicability;
use rustc_hir::def_id::DefId;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::Span;
use rustc_span::hygiene::{ExpnKind, MacroKind};

use crate::rustc_lint::LintContext;
use clippy_utils::source::snippet_opt;

declare_clippy_lint! {
    /// ### What it does
    /// Checks that common macros are used with consistent bracing.
    ///
    /// ### Why is this bad?
    /// Having non-conventional braces on well-stablished macros can be confusing
    /// when debugging, and they bring incosistencies with the rest of the ecosystem.
    ///
    /// ### Example
    /// ```no_run
    /// vec!{1, 2, 3};
    /// ```
    /// Use instead:
    /// ```no_run
    /// vec![1, 2, 3];
    /// ```
    #[clippy::version = "1.55.0"]
    pub NONSTANDARD_MACRO_BRACES,
    correctness,
    "check consistent use of braces in macro"
}

impl_lint_pass!(MacroBraces => [NONSTANDARD_MACRO_BRACES]);

pub struct MacroBraces {
    macro_braces: (FxHashMap<String, (char, char)>, usize),
}

impl MacroBraces {
    pub fn new(conf: &'static Conf) -> Self {
        Self {
            macro_braces: macro_braces(&conf.standard_macro_braces),
        }
    }
}

impl EarlyLintPass for MacroBraces {
    fn check_mac(&mut self, cx: &EarlyContext<'_>, mac: &ast::MacCall) {
        // dbg!(snippet_opt(cx.sess(), mac.span()), mac.span());
        if let Some(last_segment) = mac.path.segments.last()
            && let name = last_segment.ident.as_str()
            && let Some(&braces) = self.macro_braces.0.get(name)
            && let Some(snip) = snippet_opt(cx.sess(), mac.span().with_lo(last_segment.span().lo()))
            && let Some(macro_args_str) = &snip.strip_prefix(name).and_then(|snip| snip.strip_prefix('!'))
            && let Some(old_open_brace @ ('{' | '(' | '[')) = macro_args_str.trim_start().chars().next()
            && old_open_brace != braces.0
        {
            emit_help(
                cx,
                &snippet_opt(cx.sess(), mac.span()).unwrap(),
                braces,
                mac.span(),
                false,
            );
        }
    }

    fn check_mac_def(&mut self, cx: &EarlyContext<'_>, mac: &ast::MacroDef) {
        use rustc_ast::token::{Delimiter, Token, TokenKind};
        use rustc_ast::tokenstream::TokenTree;

        fn check_ts(cx: &EarlyContext<'_>, ts: &TokenStream, macro_braces: &FxHashMap<String, (char, char)>) {
            let ts = ts.iter().collect::<Vec<_>>();
            for (i, x) in ts.iter().enumerate() {
                let x_span = x.span();
                let span_len = x_span.hi().0 as usize - x_span.lo().0 as usize;

                // dbg!(&x);
                if let TokenTree::Delimited(_, _, Delimiter::Brace, token_stream) = x {
                    check_ts(cx, &token_stream, &macro_braces);
                    // target_tokenstream = token_stream;
                    // while let rustc_ast::tokenstream::TokenTree::Delimited(_, _, _, token_stream)
                    // = target_tokenstream {     target_tokenstream =
                    // token_stream }
                } else if let TokenTree::Token(
                    Token {
                        kind: TokenKind::Ident(tident, _),
                        span,
                    },
                    _,
                ) = x
                    && let Some(peekable) = ts.iter().nth(i + 1)
                    && let TokenTree::Token(
                        Token {
                            kind: TokenKind::Bang, ..
                        },
                        _,
                    ) = *peekable
                // && let name = tident.as_str()
                && let Some(TokenTree::Delimited(_, _, delim, _)) = ts.iter().nth(i + 2)
                && let Some(snip) = snippet_opt(cx.sess(), span.with_hi(ts.iter().nth(i + 2).unwrap().span().hi()))
                && let Some(&braces) = macro_braces.get(tident.as_str())
                && let Some(old_open_brace) = match delim {
                    Delimiter::Brace => Some('{'),
                    Delimiter::Parenthesis => Some('('),
                    Delimiter::Bracket => Some('['),
                    _ => None,


                }
                && old_open_brace != braces.0
                {
                    emit_help(
                        cx,
                        &snip,
                        braces,
                        span.with_hi(ts.iter().nth(i + 2).unwrap().span().hi()),
                        false,
                    );
                    // dbg!(peekable);
                }

                // if span_len >= 3 && span_len <= self.macro_braces.1 {
                // dbg!(snippet_opt(cx.sess(), x.span()));
                // }
            }
        }

        if mac.macro_rules {
            check_ts(&cx, &mac.body.tokens, &self.macro_braces.0);
        }
    }
}

fn emit_help(cx: &EarlyContext<'_>, snip: &str, (open, close): (char, char), span: Span, add_semi: bool) {
    let semi = if add_semi { ";" } else { "" };
    if let Some((macro_name, macro_args_str)) = snip.split_once('!') {
        let mut macro_args = macro_args_str.trim().to_string();
        // now remove the wrong braces
        macro_args.pop();
        macro_args.remove(0);
        span_lint_and_sugg(
            cx,
            NONSTANDARD_MACRO_BRACES,
            span,
            format!("use of irregular braces for `{macro_name}!` macro"),
            "consider writing",
            format!("{macro_name}!{open}{macro_args}{close}{semi}"),
            Applicability::MachineApplicable,
        );
    }
}

fn macro_braces(conf: &[MacroMatcher]) -> (FxHashMap<String, (char, char)>, usize) {
    let mut braces = FxHashMap::from_iter(
        [
            ("print", ('(', ')')),
            ("println", ('(', ')')),
            ("eprint", ('(', ')')),
            ("eprintln", ('(', ')')),
            ("write", ('(', ')')),
            ("writeln", ('(', ')')),
            ("format", ('(', ')')),
            ("format_args", ('(', ')')),
            ("vec", ('[', ']')),
            ("matches", ('(', ')')),
        ]
        .map(|(k, v)| (k.to_string(), v)),
    );
    // We want users items to override any existing items
    for it in conf {
        braces.insert(it.name.clone(), it.braces);
    }

    let max_len = braces.iter().fold(11, |max_len, macro_name| {
        if macro_name.0.len() > max_len {
            macro_name.0.len()
        } else {
            max_len
        }
    });

    (braces, max_len)
}
