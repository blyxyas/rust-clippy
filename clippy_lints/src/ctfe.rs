use clippy_utils::msrvs::{msrv_check_attributes, msrv_check_attributes_post};
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::Span;

declare_lint_pass! {
    /// Ensures that Constant-time Function Evaluation is being done (specifically, MIR lint passes).
    /// As Clippy deactivates codegen, this lint ensures that CTFE (used in hard errors) is still ran.
    ClippyCtfe => []
}

impl<'tcx> LateLintPass<'tcx> for ClippyCtfe {
    fn check_fn(
        &mut self,
        cx: &LateContext<'_>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        _: Span,
        defid: LocalDefId,
    ) {
        cx.tcx.ensure().mir_drops_elaborated_and_const_checked(defid); // Lint
    }

    fn check_attributes(&mut self, cx: &LateContext<'_>, attrs: &[rustc_ast::ast::Attribute]) {
        let sess = rustc_lint::LintContext::sess(cx);
        msrv_check_attributes(sess, attrs);
    }

    fn check_attributes_post(&mut self, cx: &LateContext<'_>, attrs: &[rustc_ast::ast::Attribute]) {
        let sess = rustc_lint::LintContext::sess(cx);
        msrv_check_attributes_post(sess, attrs);
    }
}
