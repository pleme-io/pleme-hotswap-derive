{
  # CORRECTED 2026-08-06. This read "The derive macro itself is NOT yet
  # implemented ... deliberately not rushed" — which was true when written and
  # has been false since the emitter landed. Measured: pleme-hotswap-derive/
  # src/lib.rs is 169 lines carrying a real #[proc_macro_derive(HotSwap, …)]
  # that emits `const FIELD_CLASSES` + `fn classify_change`, with four
  # compile_error!() arms, and tests/ carries 5 behavioural tests plus 2
  # trybuild UI fixtures with committed .stderr. A description saying "not
  # implemented" on a crate published to crates.io is worse than no
  # description: it is the first thing a prospective consumer reads, and it
  # tells them not to adopt a macro that works.
  description = "Per-field compile-time-exhaustive hot-swap-safety classification for shikumi TieredConfig structs -- #[derive(HotSwap)] with #[hot_swap] / #[restart_required(reason=\"...\")] tagging, per theory/CALHA.md SS4/SS6.1/SS6.2. Every named field must carry exactly one tag; an untagged or double-tagged field is a compile_error!(). The derive emits `const FIELD_CLASSES: &[(&str, HotSwapClass)]` and `fn classify_change(&self, new: &Self) -> SwapDecision`, returning Free when every changed field is hot-swappable and RequiresRestart(reasons) otherwise -- so 'can this config change be applied to a running process?' is answered by the type system rather than by a runbook. Ships alongside pleme-hotswap, the dependency-free runtime crate holding HotSwapClass / SwapDecision / HotSwapClassifier.";
  inputs = {
    nixpkgs = {
      follows = "substrate/nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    substrate = {
      url = "github:pleme-io/substrate";
    };
    # `outputs` destructures crate2nix and `lib/build/rust/library.nix`
    # documents `crate2nix = inputs.crate2nix` as required — but it was never
    # declared here, so this flake could not have evaluated even with the
    # syntax below fixed. Follows substrate's own pin rather than taking a
    # second one: substrate's header states its pin is meant to propagate, and
    # two crate2nix revisions in one graph is the diamond that shape avoids.
    crate2nix = {
      follows = "substrate/crate2nix";
    };
  };
  outputs = { self, nixpkgs, flake-utils, substrate, crate2nix, ... }:
    # `eachDefaultSystem (system: …)` — the argument is a FUNCTION, so it
    # needs its own parens. Without them this parsed as `eachDefaultSystem
    # system` followed by a stray `:`, which is a syntax error: the flake had
    # never evaluated, on any machine, since it was written.
    flake-utils.lib.eachDefaultSystem (system:
      let
        rustLibrary = import "${substrate}/lib/rust-library.nix" {
          inherit system nixpkgs;
          nixLib = substrate;
          inherit crate2nix;
        };
      in
      # `rustLibrary { … }` returns the whole `{ packages, devShells, apps,
      # checks }` outputs attrset, not a derivation — so the previous
      # `packages.default = lib` nested that attrset under `packages.default`
      # and produced `packages.default.name is not a derivation`. Returning it
      # directly is the documented shape, and it is what gives this repo
      # `checks.tests` (the crate2nix `runTests` surface) — which `nix flake
      # check` actually builds, where `packages` are only evaluated.
      rustLibrary {
        name = "pleme-hotswap-derive";
        src = ./.;
      });
}
