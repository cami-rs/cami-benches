#![feature(trait_alias)]
#![feature(is_sorted)]
#![feature(extend_one)]

//#![allow(warnings, unused)]
use cami::prelude::*;
use core::iter;
use criterion::{criterion_group, Criterion};
use fastrand::Rng;
use lib_benches::*;

#[path = "shared/lib_benches.rs"]
mod lib_benches;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type IdState = ();

    fn generate_item(rng: &mut Rng, total_length: &mut IdState) -> Vec<u8> {
        let item_len = rng.usize(..MAX_ITEM_LEN);
        let mut item = Vec::<u8>::with_capacity(item_len);
        item.extend(iter::repeat_with(|| rng.u8(..)).take(item_len));
        item
    }

    fn id_postfix(_: &IdState) -> String {
        String::with_capacity(0)
    }

    let mut id_state: IdState = ();

    bench_vec_sort_bin_search::<
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
        //|own| &own[..]
        |own| &own[..],
    );
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_target
}
// Based on expansion of `criterion_main!(benches);`
fn main() {
    benches();

    Criterion::default().configure_from_args().final_summary();
}
