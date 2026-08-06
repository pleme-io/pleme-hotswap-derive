{
  description = "Per-field compile-time-exhaustive hot-swap-safety classification for shikumi TieredConfig structs -- #[derive(HotSwap)] with #[hot_swap] / #[restart_required(reason=\"...\")] tagging, per theory/CALHA.md SS4/SS6.1/SS6.2. The derive macro itself is NOT yet implemented (it needs a real, non-trivial extension to tatara-rust-ast's PerFieldDeriveSpec emitter -- exhaustive multi-tag dispatch + attribute-argument extraction + compile_error!() on an untagged field -- confirmed to touch ~13 files there, deliberately not rushed). This repo ships the real, versioned target API (HotSwapClass, SwapDecision, HotSwapClassifier) plus a derive entry point that fails loudly (compile_error!()) rather than silently doing the wrong thing.";
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
