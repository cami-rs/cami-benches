#![feature(trait_alias)]
#![feature(is_sorted)]
#![feature(extend_one)]

use cami::prelude::*;
use core::iter;
use criterion::Criterion;
use fastrand::Rng;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use lib_benches::outish::{OutCollectionVec, OutCollectionVecIndicator, OutIndicatorStrIndicator};
use lib_benches::shared::MAX_ITEM_LEN;
use lib_benches::shared_iai::DataOut;
use once_cell::sync::OnceCell;

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

type OutType = &'static str;
type OutTypeVec = Vec<OutType>;
type OutCollectionVecStr = OutCollectionVec<'static, &'static str>;
type OutCollectionVecCamiStr = OutCollectionVec<'static, Cami<&'static str>>;
fn out() -> &'static DataOut<'static, OutType, OutCollectionVecStr, OutCollectionVecCamiStr> {
    static OUT: OnceCell<DataOut<&str, OutCollectionVecStr, OutCollectionVecCamiStr>> =
        OnceCell::new();
    OUT.get_or_init(|| {
        static OWN: OnceCell<OutTypeVec> = OnceCell::new();
        let own = OWN.get_or_init(|| lib_benches::shared_iai::data_own(|_rnd| ""));

        lib_benches::shared_iai::data_out(own, |item| *item)
    })
}

#[library_benchmark]
#[bench::short(10)]
#[bench::long(30)]
#[bench::b1(0)]
#[bench::b2(5)]
fn bench1(value: u64) -> u64 {
    core::hint::black_box(value)
}

#[library_benchmark]
#[bench::b1()]
#[bench::b2()]
fn bench2() {
    core::hint::black_box(1);
}

library_benchmark_group!(
    name = bench_group;
    compare_by_id = true;
    benchmarks = bench1, bench2
);

main!(library_benchmark_groups = bench_group);
