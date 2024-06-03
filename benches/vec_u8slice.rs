#![feature(trait_alias)]
#![feature(is_sorted)]
#![feature(extend_one)]

//#![allow(warnings, unused)]
use cami::prelude::*;
use core::iter;
use criterion::{criterion_group, Criterion};
use fastrand::Rng;
use lib_benches::*;

extern crate alloc;

#[path = "shared/lib_benches.rs"]
mod lib_benches;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type IdState = usize;

    fn generate_item(rng: &mut Rng, total_length: &mut IdState) -> Vec<u8> {
        let item_len = rng.usize(..MAX_ITEM_LEN);
        let mut item = Vec::<u8>::with_capacity(item_len);
        item.extend(iter::repeat_with(|| rng.u8(..)).take(item_len));

        *total_length += item.len();
        item
    }

    fn id_postfix(total_length: &IdState) -> String {
        format!("Sum len: {total_length}.")
    }

    let mut total_length: IdState = 0;

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
        &mut total_length,
        id_postfix,
        generate_item,
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
