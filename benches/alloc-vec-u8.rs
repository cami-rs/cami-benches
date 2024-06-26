#![feature(trait_alias)]
#![feature(is_sorted)]
#![feature(extend_one)]
//#![cfg_attr(all(feature = "unsafe", feature = "nightly"), feature(anonymous_lifetime_in_impl_trait))]
// https://github.com/rust-lang/rust/issues/52662
//
//#![cfg_attr(all(feature = "unsafe", feature = "nightly"), feature(associated_type_bounds))]
//#![feature(type_alias_impl_trait)]

use cami::prelude::*;
use cami_benches::outish::{OutCollectionVecIndicator, OutIndicatorNonRefIndicator};
use criterion::{criterion_group, Criterion};
use fastrand::Rng;

extern crate alloc;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type IdState = ();

    fn id_postfix(_: &IdState) -> String {
        String::new()
    }

    let mut id_state: IdState = ();
    lib_benches::shared::bench_vec_sort_bin_search::<
        u8,
        u8,
        OutIndicatorNonRefIndicator,
        OutCollectionVecIndicator,
        Rng,
        IdState,
    >(
        c,
        &mut rng,
        "u8",
        &mut id_state,
        id_postfix,
        |rng, _| rng.u8(..),
        |own| *own,
    );
}

criterion_group! {
    name = benches;
    config = lib_benches::criterionish::criterion_config();
    targets = bench_target
}
// Based on expansion of `criterion_main!(benches);`
fn main() {
    benches();

    Criterion::default().configure_from_args().final_summary();
}
