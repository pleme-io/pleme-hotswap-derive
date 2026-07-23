//! `#[derive(HotSwap)]` — per `theory/CALHA.md` §4/§6.1/§6.2.
//!
//! **This derive is a documented, deliberate `compile_error!()` stub — it is
//! not implemented.** Per `theory/CALHA.md`'s own Corrections Ledger #5, the
//! real implementation is a materially larger extension of
//! `tatara-rust-ast`'s `PerFieldDeriveSpec` emitter than a single new field:
//! it needs multi-tag dispatch (not today's single opt-in
//! `field_attribute`), attribute-argument extraction (reading
//! `#[restart_required(reason = "...")]`'s `reason` string, which today's
//! emitter has no path for at all), and exhaustiveness enforcement (a
//! `compile_error!()` when a field carries none of the declared tags —
//! today's emitter silently drops an untagged field from the generated
//! impl, which is the exact opposite of the safety guarantee this design
//! needs). Confirmed to touch ~13 files in `tatara-rust-ast` — real,
//! scoped, deliberately deferred rather than rushed.
//!
//! Shipping a *working* single-tag stand-in instead of this stub was
//! considered and rejected: today's `field_attribute` mechanism silently
//! excludes an untagged field from `HotSwapClassifier::classify_change`'s
//! diff entirely, which would mean a config field nobody remembered to tag
//! produces **no decision at all** for a live-affecting change — silently
//! worse than either declared arm (`Free` swaps it, `RequiresRestart`
//! forces a safe restart). A macro that compiles clean but is quietly
//! unsafe is worse than one that refuses to compile at all, so this crate
//! refuses, loudly, with a message pointing here.
//!
//! See [`pleme_hotswap`] for the real, stable runtime types
//! (`HotSwapClass`, `SwapDecision`, `HotSwapClassifier`) this derive will
//! generate impls against once built.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HotSwap, attributes(hot_swap, restart_required))]
pub fn derive_hot_swap(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let message = format!(
        "#[derive(HotSwap)] on `{name}` is not yet implemented. \
         See https://github.com/pleme-io/pleme-hotswap-derive#status and \
         theory/CALHA.md SS4/SS6.1/SS6.2 in pleme-io/theory for the design \
         and why this refuses to compile instead of silently under-classifying \
         an untagged field."
    );

    quote! {
        compile_error!(#message);
    }
    .into()
}
