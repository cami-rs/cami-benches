# Cami Benchmarks

## Purpose

Benchmarks of (public API only of) Rust crate [cami-rs/cami](https://github.com/cami-rs/cami).

Currently benchmarks require `alloc` (but <!-- TODO: some of them do -->they CAN run `no_std` part
of `cami` only).

## Conventions

The first part of a benchmarkname indicates the collection/storage being used. That is the
collection/storage of "out" items, ones being sorted or searched for - not necessarily the same kind
of collection/storage as used for "own" items (in case the "out" items refer to/borrow from "own"
items).

The second part indicates the item type.

For example
- [benches/vec_string.rs](benches/vec_string.rs) stores (owned) `String` instances in a `Vec`
- [benches/vec_u8slice.rs](benches/vec_u8slice.rs) stores a (borrowed) `&[u8]` (slices of bytes,
  `u8`), in a `Vec`.
- [benches/btreeset_u8.rs](benches/btreeset_u8.rs) stores bytes (`u8`) in an
  [alloc::collections::BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree_set/struct.BTreeSet.html).

## How to run

@TODO UPDATE

You **must** specify any features needed/used by the appropriate bench. Even if a chosen bench
declares such feature(s) as required (in `required-features` in [Cargo.toml](Cargo.toml)),
specifying the bench as with `cargo bench --bench ...` or `cargo check --bench ...` will **not**
run/check that bench, unless you specify all features that it requires.

Relevant features (ones that affect any benches; this list may change):

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
- `cargo check --bench vec_string  --features alloc`
- `cargo check --bench vec_string  --features alloc,deref_pure`
- `cargo bench --bench vec_string  --features alloc`
- `cargo bench --bench vec_string  --features alloc,deref_pure`
-
- `cargo check --bench vec_str     --features alloc`
- `cargo check --bench vec_str     --features alloc,deref_pure`
- `cargo bench --bench vec_str     --features alloc`
- `cargo bench --bench vec_str     --features alloc,deref_pure`
-
- `cargo check --bench btreeset_u8 --features alloc`
- `cargo check --bench btreeset_u8 --features alloc,deref_pure`
- `cargo bench --bench btreeset_u8 --features alloc`
- `cargo bench --bench btreeset_u8 --features alloc,deref_pure`
-
- `cargo check --benches           --features alloc`
- `cargo check --benches           --features alloc,deref_pure`
- `cargo bench                     --features alloc`
- `cargo bench                     --features alloc,deref_pure`

`alloc` is required by the benches. But, because `alloc` is not a default feature in `cami`, those
benches won't be run until you specify it.
