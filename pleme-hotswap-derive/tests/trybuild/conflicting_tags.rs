use pleme_hotswap_derive::HotSwap;

#[allow(unused_imports)]
use pleme_hotswap::{HotSwapClass, HotSwapClassifier, SwapDecision};

#[derive(HotSwap)]
struct Config {
    // Carries BOTH tags -- exactly one is required per field.
    #[hot_swap]
    #[restart_required(reason = "bound at process start")]
    bind_addr: String,
}

fn main() {}
