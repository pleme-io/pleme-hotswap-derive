//! Regenerates `pleme-hotswap-derive/src/lib.rs` from a typed
//! `PerFieldDeriveSpec` via `tatara-rust-ast`'s `field_tag` aggregate
//! emission mode (`tatara-rust-ast @ 196e76f`).
//!
//! Only `src/lib.rs` is overwritten -- the hand-authored `Cargo.toml`
//! (package metadata, dev-dependencies for the real tests) is left
//! alone; `compile_to_crate`'s own generic `Cargo.toml` output is
//! discarded. The spec itself is persisted alongside the generated
//! output as `generated.spec.json`, per the org's CLOSED-LOOP
//! MASS-SYNTHESIS rule 3 (persist the spec next to what it generated,
//! so `git diff` after a forced re-render proves determinism and a
//! reader can audit "where did this file come from?" without leaving
//! the repo).
//!
//! Run: `cargo run -p pleme-hotswap-gen` from the workspace root.

use std::path::PathBuf;

use tatara_rust_ast::{CompileToCrate, Ident};
use tatara_rust_derive::{AggregateSpec, FieldTag, PerFieldDeriveSpec, PerFieldTarget, TagSpec};

/// The real `#[derive(HotSwap)]` spec: aggregate mode, matching
/// `pleme_hotswap::HotSwapClassifier`'s `const FIELD_CLASSES` + `fn
/// classify_change` exactly (same shape as
/// `tatara-rust-examples::hot_swap_classifier_spec`, with `trait_name`
/// renamed to match this crate's actual `#[proc_macro_derive(HotSwap,
/// attributes(hot_swap, restart_required))]`).
fn hot_swap_derive_spec() -> PerFieldDeriveSpec {
    PerFieldDeriveSpec {
        trait_name: Ident::new("HotSwap"),
        target: PerFieldTarget::NamedStruct,
        trait_ref: Some("HotSwapClassifier".into()),
        per_field_template: String::new(), // unused -- aggregate mode
        method_name_template: None,
        impl_prelude: None,
        skip_fields: vec![],
        field_attribute: None,
        field_tag: Some(TagSpec {
            exhaustive: true,
            tags: vec![
                FieldTag {
                    name: "hot_swap".into(),
                    required_args: vec![],
                    per_field_template: String::new(),
                    aggregate_const_entry: Some(
                        "(stringify!(#field_name), HotSwapClass::Free),".into(),
                    ),
                    // A Free field changing needs no statement at all --
                    // the default (empty reasons -> SwapDecision::Free)
                    // already covers it.
                    aggregate_stmt: Some(String::new()),
                },
                FieldTag {
                    name: "restart_required".into(),
                    required_args: vec!["reason".into()],
                    per_field_template: String::new(),
                    aggregate_const_entry: Some(
                        "(stringify!(#field_name), HotSwapClass::RequiresRestart { reason: #reason }),".into(),
                    ),
                    aggregate_stmt: Some(
                        "if self.#field_name != new.#field_name { reasons.push(#reason); }".into(),
                    ),
                },
            ],
            aggregate: Some(AggregateSpec {
                const_signature: "const FIELD_CLASSES: &'static [(&'static str, HotSwapClass)] = ".into(),
                method_signature: "fn classify_change(&self, new: &Self) -> SwapDecision".into(),
                method_setup: "let mut reasons: Vec<&'static str> = Vec::new();".into(),
                method_return: concat!(
                    "if reasons.is_empty() { SwapDecision::Free } ",
                    "else { SwapDecision::RequiresRestart(reasons) }"
                )
                .into(),
            }),
        }),
    }
}

fn main() {
    let spec = hot_swap_derive_spec();

    let scaffold = spec
        .compile_to_crate("pleme-hotswap-derive")
        .expect("hot_swap_derive_spec must compile to a crate scaffold");

    let files = scaffold.to_files();
    let lib_rs = files
        .get("src/lib.rs")
        .expect("compile_to_crate always emits src/lib.rs");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let derive_crate_dir = manifest_dir
        .parent()
        .expect("gen/ has a parent")
        .join("pleme-hotswap-derive");

    let lib_rs_path = derive_crate_dir.join("src/lib.rs");
    std::fs::write(&lib_rs_path, lib_rs).expect("write generated src/lib.rs");
    println!("wrote {}", lib_rs_path.display());

    let spec_json =
        serde_json::to_string_pretty(&spec).expect("PerFieldDeriveSpec must serialize");
    let spec_path = derive_crate_dir.join("generated.spec.json");
    std::fs::write(&spec_path, format!("{spec_json}\n")).expect("write generated.spec.json");
    println!("wrote {}", spec_path.display());
}
