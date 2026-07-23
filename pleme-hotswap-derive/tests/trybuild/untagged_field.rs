use pleme_hotswap_derive::HotSwap;

// `pleme_hotswap` types are unqualified in the derive's expansion (the
// consumer is expected to `use pleme_hotswap::{HotSwapClass, SwapDecision,
// HotSwapClassifier};`), but this fixture must fail before ever reaching
// that expansion -- the exhaustiveness check fires on `bind_addr` before
// any of these names would need to resolve.
#[allow(unused_imports)]
use pleme_hotswap::{HotSwapClass, HotSwapClassifier, SwapDecision};

#[derive(HotSwap)]
struct Config {
    #[hot_swap]
    log_level: String,
    // No #[hot_swap] / #[restart_required(...)] tag -- the exhaustiveness
    // guarantee (theory/CALHA.md SS5.1) requires this to fail loudly
    // instead of silently producing no decision for this field's changes.
    bind_addr: String,
}

fn main() {}
