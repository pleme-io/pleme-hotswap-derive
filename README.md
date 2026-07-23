# pleme-hotswap-derive

> Per-field compile-time-exhaustive hot-swap-safety classification for shikumi `TieredConfig` structs -- `#[derive(HotSwap)]` with `#[hot_swap]` / `#[restart_required(reason="...")]` tagging, per [`theory/CALHA.md`](https://github.com/pleme-io/theory/blob/main/CALHA.md) §4/§6.1/§6.2.

## Status (tier-honest, never rounded up)

**The derive macro is real and working.** `#[derive(HotSwap)]` generates a
`pleme_hotswap::HotSwapClassifier` impl -- one `const FIELD_CLASSES` +
one `classify_change` method spanning every field -- from a struct where
each field carries exactly one of `#[hot_swap]` / `#[restart_required(reason
= "...")]`. Both safety guarantees are real and verified end to end, not
just claimed:

- **Exhaustiveness** -- an untagged field is a `compile_error!()`, not a
  silently-dropped field (`tests/trybuild/untagged_field.rs`).
- **Uniqueness** -- a field carrying both tags is also a `compile_error!()`
  (`tests/trybuild/conflicting_tags.rs`).
- **Correctness** -- `tests/derive_works.rs` applies the derive to a real
  two-field struct and proves `FIELD_CLASSES` introspection plus
  `classify_change` across all four scenarios (no change / only the Free
  field changed / only the restart-required field changed / both changed).

Generated, not hand-written: the derive's `src/lib.rs` is emitted by
`tatara-rust-ast`'s `PerFieldDeriveSpec` (`field_tag`'s aggregate mode,
`tatara-rust-ast @ 196e76f`) via the `gen/` crate in this workspace
(`cargo run -p pleme-hotswap-gen`, run from the workspace root). The exact
spec used is committed alongside the generated output at
`pleme-hotswap-derive/generated.spec.json` for auditability and
regeneration -- per the org's persist-the-spec-next-to-the-artifact
convention, `git diff` after a forced re-render proves the generator is
deterministic. **Do not hand-edit `pleme-hotswap-derive/src/lib.rs`** --
regenerate it from the spec in `gen/src/main.rs` instead.

**What's still not done:** `shikumi::hotswap` (the `ArcSwap`-backed
hot-swap config store + `Validate` trait this derive's output is meant to
feed into, per `theory/CALHA.md` §6.3) remains unbuilt. This crate proves
the classification half of the design; the live config-store half is a
separate, further increment.

## Building

```bash
cargo build --workspace --all-targets
cargo test --workspace
```

## Regenerating the derive

```bash
cargo run -p pleme-hotswap-gen
git diff pleme-hotswap-derive/src/lib.rs pleme-hotswap-derive/generated.spec.json
```

A clean diff after a forced re-render is the determinism proof.

## License

MIT.
