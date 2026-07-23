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
  };
  outputs = { self, nixpkgs, flake-utils, substrate, crate2nix, ... }:
    flake-utils.lib.eachDefaultSystem system:
      let
        rustLibrary = import "${substrate}/lib/rust-library.nix" {
          inherit system nixpkgs;
          nixLib = substrate;
          inherit crate2nix;
        };
        lib = rustLibrary {
          name = "pleme-hotswap-derive";
          src = ./.;
        };
      in {
        packages.default = lib;
      };
}
