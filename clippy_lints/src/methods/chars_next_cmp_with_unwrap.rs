use crate::HVec;

use super::CHARS_NEXT_CMP;
use rustc_lint::LateContext;
/// Checks for the `CHARS_NEXT_CMP` lint with `unwrap()`.
pub(super) fn check(cx: &LateContext<'_>, info: &crate::methods::BinaryExprInfo<'_>) -> bool {
    crate::methods::chars_cmp_with_unwrap::check(cx, info, &["chars", "next", "unwrap"], CHARS_NEXT_CMP, "starts_with")
}
