#![feature(extend_one)]
#![feature(is_sorted)]
#![feature(thread_id_value)]
#![feature(trait_alias)]

use crate::lib_benches::shared;
use cami::prelude::*;
use core::iter;
use fastrand::Rng;
use iai_callgrind::{library_benchmark, library_benchmark_group, main, LibraryBenchmarkConfig};
use lib_benches::outish::{OutCollectionVec, OutCollectionVecIndicator, OutIndicatorStrIndicator};
use lib_benches::shared::{DataOwnAndOut, MAX_ITEM_LEN};
use lib_benches::shared_iai;
use once_cell::sync::OnceCell;

mod lib_benches;

type OwnType = String;
type OutType = &'static str;

fn own_and_out() -> &'static DataOwnAndOut<OwnType, OutType> {
    eprintln!("own_and_out()");
    eprintln!(
        "process ID: {}, thread ID: {}",
        std::process::id(),
        std::thread::current().id().as_u64()
    );
    static OUT_AND_OWN: OnceCell<DataOwnAndOut<OwnType, OutType>> = OnceCell::new();
    OUT_AND_OWN.get_or_init(|| {
        eprintln!("own_and_out() generating owned & out data");
        DataOwnAndOut::new(|_rnd| "".to_owned(), |string| &string[..], true)
    })
}

fn own() -> &'static [OwnType] {
    own_and_out().own
}

fn out() -> &'static [OutType] {
    &own_and_out().out
}

//------

#[library_benchmark]
#[bench::sort_stable()]
fn sort_stable_lexi() {
    core::hint::black_box(shared::lexi_stable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()));
}

#[library_benchmark]
#[bench::sort_stable()]
fn sort_stable_cami() {
    core::hint::black_box(shared::cami_stable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()));
}
//------

#[library_benchmark]
#[bench::sort_unstable()]
fn sort_unstable_lexi() {
    core::hint::black_box(shared::lexi_unstable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()));
}

#[library_benchmark]
#[bench::sort_unstable()]
fn sort_unstable_cami() {
    core::hint::black_box(shared::cami_unstable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()));
}

#[library_benchmark]
fn return_result() -> String {
    core::hint::black_box("from_ret.._res..".to_owned())
}

//------

library_benchmark_group!(
    name = bench_group;
    config = LibraryBenchmarkConfig::default().pass_through_envs(["RND_SEED_DEC", "RND_SEED_HEX"]);
    compare_by_id = true;
    benchmarks = sort_stable_lexi, sort_stable_cami, sort_unstable_lexi, sort_unstable_cami, return_result
);

main!(library_benchmark_groups = bench_group);
