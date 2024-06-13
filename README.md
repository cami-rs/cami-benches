# Cami Benchmarks

## Purpose

Benchmarks of (public API only of) Rust crate [cami-rs/cami](https://github.com/cami-rs/cami).

## Naming Conventions

Name of each bench has three parts, separated with `-`. The parts indicate:
- whether it uses `alloc` (or not), or `std` (if it's `std`, that implies `alloc`). That pts benches into three groups:
  - `stack-...`: `no_std`-compatible, and no allocation,
  - `alloc-...`: `no_std`-compatible, but requiring `alloc`,
  - `stdlb-`: requires `std`;
- the collection/storage being used. That is the collection/storage of "out" items, ones being
sorted or searched for - not necessarily the same kind of collection/storage as used for "own" items
(in case the "out" items refer to/borrow from "own" items).
- the item type.

For example
- [benches/alloc-vec-string.rs](benches/alloc-vec-string.rs) stores (owned) `String` instances in a `Vec`
- [benches/alloc-vec-u8_slice.rs](benches/alloc-vec-u8_slice.rs) stores a (borrowed) `&[u8]` (slices of bytes,
  `u8`), in a `Vec`.
- [benches/alloc-btreeset-u8.rs](alloc-btreeset-u8.rs) stores bytes (`u8`) in an
  [alloc::collections::BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree_set/struct.BTreeSet.html).

`stack-*` benches **do** use `alloc`, but only for their own operation. These benches allocate a
`Vec`, but before calling `cami` they convert that `Vec` into a slice. They invoke `cami`'s
allocation-free functionality only.

## How to run

You **must** specify any features required by the appropriate bench. Even if a chosen bench declares
such feature(s) as necessary (in `required-features` in [Cargo.toml](Cargo.toml)), specifying such a
bench, as with `cargo bench --bench ...` or `cargo check --bench ...`, will **not** "match" that
bench (will not run/check it), unless you specify all features that it requires.

Relevant features:

- `alloc` - required for `alloc-...` benches, and
- `std` - for `std-...` benches - currently: none,
- `deref_pure` is optional.

Invoke `cargo bench` or `cargo check --benches` like:
```bash
cargo check --bench stack-slice-u8
cargo check --bench stack-slice-u8         --features deref_pure
cargo bench --bench stack-slice-u8
cargo bench --bench stack-slice-u8         --features deref_pure

cargo check --bench alloc-vec-u8           --features alloc
cargo check --bench alloc-vec-u8           --features alloc,deref_pure
cargo bench --bench alloc-vec-u8           --features alloc
cargo bench --bench alloc-vec-u8           --features alloc,deref_pure

cargo check --bench alloc-vec-u8_slice     --features alloc
cargo check --bench alloc-vec-u8_slice --features alloc,deref_pure
cargo bench --bench alloc-vec-u8_slice --features alloc
cargo bench --bench alloc-vec-u8_slice --features alloc,deref_pure

cargo check --bench alloc-vec-str     --features alloc
cargo check --bench alloc-vec-str     --features alloc,deref_pure
cargo bench --bench alloc-vec-str     --features alloc
cargo bench --bench alloc-vec-str     --features alloc,deref_pure

cargo check --bench alloc-vec-string  --features alloc
cargo check --bench alloc-vec-string  --features alloc,deref_pure
cargo bench --bench alloc-vec-string  --features alloc
cargo bench --bench alloc-vec-string  --features alloc,deref_pure

cargo check --bench alloc-btreeset-u8 --features alloc
cargo check --bench alloc-btreeset-u8 --features alloc,deref_pure
cargo bench --bench alloc-btreeset-u8 --features alloc
cargo bench --bench alloc-btreeset-u8 --features alloc,deref_pure

cargo check --benches           --features alloc
cargo check --benches           --features alloc,deref_pure
cargo bench                     --features alloc
cargo bench                     --features alloc,deref_pure
```

`alloc` is required by the benches. But, because `alloc` is not a default feature in `cami`, those
benches won't be run until you specify it.
