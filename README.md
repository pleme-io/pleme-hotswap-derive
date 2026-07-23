# pleme-hotswap-derive

> Per-field compile-time-exhaustive hot-swap-safety classification for shikumi `TieredConfig` structs -- `#[derive(HotSwap)]` with `#[hot_swap]` / `#[restart_required(reason="...")]` tagging, per [`theory/CALHA.md`](https://github.com/pleme-io/theory/blob/main/CALHA.md) §4/§6.1/§6.2.

## Status (tier-honest, never rounded up)

**The derive macro itself is a deliberate `compile_error!()` stub, not an
implementation.** `#[derive(HotSwap)]` on any struct fails to compile, with
a message pointing here. This is intentional, verified behavior (a
`trybuild` test proves it), not a placeholder that got shipped by mistake.

Why: the real implementation needs a materially larger extension of
`tatara-rust-ast`'s `PerFieldDeriveSpec` emitter than "add one field" —
multi-tag dispatch (today's `field_attribute` is single-tag, opt-in-only),
attribute-argument extraction (reading a `reason = "..."` string — today's
emitter has no path for this at all), and real exhaustiveness enforcement (a
`compile_error!()` when a field carries none of the declared tags — today's
emitter silently *drops* an untagged field from the generated impl, which is
the opposite of the safety guarantee this design needs). Confirmed by direct
source read to touch ~13 files in `tatara-rust-ast`. Shipping a working
single-tag stand-in instead was considered and rejected: it would silently
produce **no decision at all** for an unremembered field's change, which is
worse than either real arm (`Free` swaps it, `RequiresRestart` forces a safe
restart) — a macro that compiles clean but is quietly unsafe is worse than
one that refuses to compile.

**What IS real here today:**

- `pleme-hotswap` — the runtime types (`HotSwapClass`, `SwapDecision`, the
  `HotSwapClassifier` trait), stable and buildable, so downstream design
  (including [`calha`](https://github.com/pleme-io/calha), which already
  depends on this shape) can be written against them ahead of the derive
  landing.
- `pleme-hotswap-derive` — the proc-macro crate, compiles clean, and its
  `compile_error!()` refusal is itself tested (`cargo test -p
  pleme-hotswap-derive`, a `trybuild` fixture).

**Next step, if picked up:** the `tatara-rust-ast` `PerFieldDeriveSpec`
extension named above — not started here.

## Building

```bash
cargo build --workspace --all-targets
cargo test --workspace
```

## License

MIT.
