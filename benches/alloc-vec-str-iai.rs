#![feature(trait_alias)]
#![feature(is_sorted)]
#![feature(extend_one)]

use cami::prelude::*;
use core::iter;
use criterion::{criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use lib_benches::outish::{OutCollectionVecIndicator, OutIndicatorStrIndicator, MAX_ITEM_LEN};

//#[path = "shared/lib_benches.rs"]
mod lib_benches;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type IdState = usize;

    fn generate_item(rng: &mut Rng, total_length: &mut IdState) -> String {
        let item_len = rng.usize(..MAX_ITEM_LEN);
        let mut item = Vec::<char>::with_capacity(item_len);
        item.extend(iter::repeat_with(|| rng.char(..)).take(item_len));

        let mut string = String::with_capacity(4 * item_len);
        string.extend(item.into_iter());
        *total_length += string.len();
        string
    }

    fn id_postfix(total_length: &IdState) -> String {
        format!("Sum len: {total_length}.")
    }

    let mut total_length: IdState = 0; // NOT in chars, but in bytes.

    lib_benches::shared::bench_vec_sort_bin_search::<
        String,
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
        Rng,
        IdState,
    >(
        c,
        &mut rng,
        "str",
        &mut total_length,
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
criterion_main!(benches);
