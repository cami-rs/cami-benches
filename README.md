# Cami Benchmarks

## Purpose

Benchmarks of (public API only of) Rust crate [cami-rs/cami](https://github.com/cami-rs/cami).

Currently benchmarks require `alloc` (but they run `no_std` part of Cami only).

## How to run

@TODO UPDATE

You **must** specify any optional features used by the appropriate bench. Relevant features (ones that affect any benches; this list may change):

- `alloc` - this is the only feature that is required currently (for benches), and
- `deref_pure` is optional.

Invoke `cargo bench` or `cargo check --benches` like:

- `cargo check --bench vec_u8      --features alloc`
- `cargo check --bench vec_u8      --features alloc,deref_pure`
- `cargo bench --bench vec_u8      --features alloc`
- `cargo bench --bench vec_u8      --features alloc,deref_pure`
-
- `cargo check --bench vec_u8slice --features alloc`
- `cargo check --bench vec_u8slice --features alloc,deref_pure`
- `cargo bench --bench vec_u8slice --features alloc`
- `cargo bench --bench vec_u8slice --features alloc,deref_pure`
-
- `cargo check --benches           --features alloc`
- `cargo check --benches           --features alloc,deref_pure`
- `cargo bench                     --features alloc`
- `cargo bench                     --features alloc,deref_pure`

`alloc` is required by the benches. But, because `alloc` is not a default feature in `cami`, those
benches won't be run until you specify it.
