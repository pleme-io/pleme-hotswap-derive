//! The real, working `#[derive(HotSwap)]` proof -- every field correctly
//! tagged compiles and behaves correctly: `FIELD_CLASSES` introspection
//! plus `classify_change` across the four scenarios that matter (no
//! change / only the Free field changed / only the restart-required
//! field changed / both changed). Mirrors
//! `tatara-rust-ast`'s `field_tag_aggregate_end_to_end.rs`, now run as
//! a first-party test of this shipped crate rather than a materialized
//! throwaway consumer.

use pleme_hotswap::{HotSwapClass, HotSwapClassifier, SwapDecision};
use pleme_hotswap_derive::HotSwap;

#[derive(Debug, Clone, PartialEq, Eq, HotSwap)]
struct Config {
    #[hot_swap]
    log_level: String,
    #[restart_required(reason = "bound at process start")]
    bind_addr: String,
}

fn base() -> Config {
    Config {
        log_level: "info".into(),
        bind_addr: "0.0.0.0:8080".into(),
    }
}

#[test]
fn field_classes_reports_both() {
    assert_eq!(Config::FIELD_CLASSES.len(), 2);
    assert_eq!(Config::FIELD_CLASSES[0], ("log_level", HotSwapClass::Free));
    assert_eq!(
        Config::FIELD_CLASSES[1],
        (
            "bind_addr",
            HotSwapClass::RequiresRestart {
                reason: "bound at process start"
            }
        )
    );
}

#[test]
fn no_change_is_free() {
    let a = base();
    let b = base();
    assert_eq!(a.classify_change(&b), SwapDecision::Free);
}

#[test]
fn only_free_field_changed_is_still_free() {
    let a = base();
    let mut b = base();
    b.log_level = "debug".into();
    assert_eq!(a.classify_change(&b), SwapDecision::Free);
}

#[test]
fn only_restart_field_changed_requires_restart_with_reason() {
    let a = base();
    let mut b = base();
    b.bind_addr = "0.0.0.0:9090".into();
    assert_eq!(
        a.classify_change(&b),
        SwapDecision::RequiresRestart(vec!["bound at process start"])
    );
}

#[test]
fn both_fields_changed_requires_restart() {
    let a = base();
    let mut b = base();
    b.log_level = "debug".into();
    b.bind_addr = "0.0.0.0:9090".into();
    assert_eq!(
        a.classify_change(&b),
        SwapDecision::RequiresRestart(vec!["bound at process start"])
    );
}
