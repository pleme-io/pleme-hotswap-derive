//! The runtime types `#[derive(HotSwap)]` (in the sibling `pleme-hotswap-derive`
//! crate) generates code against, per `theory/CALHA.md` §4/§6.2.
//!
//! **The derive macro itself is not yet implemented** — see that crate's
//! `README.md`. These types are real and stable now so downstream design
//! (and `calha`, which already depends on the shape) can be written against
//! them ahead of the derive landing.

/// A field's declared hot-swap stance. Exactly two arms — per
/// `theory/CALHA.md` §1's finding that config-sync is a binary safe/unsafe
/// question, not breathe's direction-dependent 3-arm `DisruptionClass`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotSwapClass {
    /// Safe to swap into a running process without a restart.
    Free,
    /// Requires a restart. `reason` is a **mandatory** string, per
    /// `theory/CALHA.md` §5.1: it makes "structurally cold forever" vs
    /// "contingently cold, not yet re-plumbed" an auditable, committed fact
    /// instead of tribal knowledge nobody wrote down.
    RequiresRestart { reason: &'static str },
}

/// The coarsest decision across every field that changed between two
/// resolved config values — the input to `calha`'s `plan_tick`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwapDecision {
    /// Every changed field is `Free` — safe to swap live.
    Free,
    /// At least one changed field `RequiresRestart` — carries the reasons
    /// for every such field, for the operator-visible `/healthz/config`
    /// surface.
    RequiresRestart(Vec<&'static str>),
}

/// Implemented by the `#[derive(HotSwap)]` macro. **No hand implementation
/// of this trait is supported or recommended** — the derive's exhaustiveness
/// guarantee (every field of the target struct carries a `#[hot_swap]` or
/// `#[restart_required(reason = "...")]` tag, or the derive emits a
/// `compile_error!()`) only holds when the derive itself generated the impl.
pub trait HotSwapClassifier {
    /// Every field's declared class, in declaration order — for
    /// introspection / `/healthz/config` status output.
    const FIELD_CLASSES: &'static [(&'static str, HotSwapClass)];

    /// Diff `self` (currently live) against `new` (freshly resolved);
    /// returns the coarsest decision across every field that actually
    /// changed.
    fn classify_change(&self, new: &Self) -> SwapDecision;
}
