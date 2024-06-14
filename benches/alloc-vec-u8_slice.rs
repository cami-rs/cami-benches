#![feature(trait_alias)]
#![feature(is_sorted)]
#![feature(extend_one)]

use cami::prelude::*;
use core::iter;
use criterion::{criterion_group, Criterion};
use fastrand::Rng;
use lib_benches::criterionish::{
    OutCollectionVecIndicator, OutIndicatorSliceIndicator, MAX_ITEM_LEN,
};

#[path = "shared/lib_benches.rs"]
mod lib_benches;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type IdState = ();

    fn generate_item(rng: &mut Rng, _id_state: &mut IdState) -> Vec<u8> {
        let item_len = rng.usize(..MAX_ITEM_LEN);
        let mut item = Vec::<u8>::with_capacity(item_len);
        item.extend(iter::repeat_with(|| rng.u8(..)).take(item_len));
        item
    }

    fn id_postfix(_: &IdState) -> String {
        String::new()
    }

    let mut id_state: IdState = ();

    lib_benches::criterionish::bench_vec_sort_bin_search::<
        Vec<u8>,
        u8,
        OutIndicatorSliceIndicator,
        OutCollectionVecIndicator,
        Rng,
        IdState,
    >(
        c,
        &mut rng,
        "u8slice",
        &mut id_state,
        id_postfix,
        generate_item,
        |own| &own[..],
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
