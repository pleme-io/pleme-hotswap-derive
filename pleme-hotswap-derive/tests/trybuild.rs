//! Proves the derive's exhaustiveness guarantee (theory/CALHA.md SS5.1)
//! actually fires -- a field carrying neither `#[hot_swap]` nor
//! `#[restart_required(reason = "...")]` fails to compile, rather than
//! silently producing no decision for that field's changes. The pass-case
//! (every field correctly tagged) is `tests/derive_works.rs`.

#[test]
fn untagged_field_refuses_to_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/untagged_field.rs");
}

#[test]
fn conflicting_tags_refuse_to_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/conflicting_tags.rs");
}
