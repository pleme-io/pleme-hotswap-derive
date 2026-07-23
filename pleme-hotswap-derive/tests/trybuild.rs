//! Proves the derive's `compile_error!()` stub actually fires -- not just
//! claimed in a doc comment. See `src/lib.rs` for why this refuses instead
//! of silently under-classifying an untagged field.

#[test]
fn derive_refuses_to_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/derive_refuses.rs");
}
