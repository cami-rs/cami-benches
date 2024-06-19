#![feature(extend_one)]
#![feature(is_sorted)]
#![feature(thread_id_value)]
#![feature(trait_alias)]

use cami::prelude::*;
use cami_benches::data::OwnAndOut;
use cami_benches::outish::{OutCollectionVec, OutCollectionVecIndicator, OutIndicatorStrIndicator};
use cami_benches::rnd::Random;
use cami_benches::{col, shared_iai};
use core::iter;
use fastrand::Rng;
use iai_callgrind::{library_benchmark, library_benchmark_group, main, LibraryBenchmarkConfig};
use once_cell::sync::OnceCell;

type OwnType = String;
type OutType = &'static str;

fn own_and_out() -> &'static OwnAndOut<OwnType, OutType> {
    static OUT_AND_OWN: OnceCell<OwnAndOut<OwnType, OutType>> = OnceCell::new();
    OUT_AND_OWN.get_or_init(|| {
        eprintln!("own_and_out() generating owned & out data");
        OwnAndOut::new(|rnd| rnd.string(), |string| &string[..], true)
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
#[bench::stable()]
fn stable_lexi() -> OutCollectionVec<'static, &'static str> {
    core::hint::black_box(col::lexi_stable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()))
}

#[library_benchmark]
#[bench::stable()]
fn stable_cami() -> OutCollectionVec<'static, Cami<&'static str>> {
    core::hint::black_box(col::cami_stable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()))
}
//------

#[library_benchmark]
#[bench::unstable()]
fn unstable_lexi() {
    let _result: &Vec<&str> = core::hint::black_box(col::lexi_unstable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()))
    .as_vec_ref();
}

#[library_benchmark]
#[bench::unstable()]
fn unstable_cami() {
    let _result: &Vec<Cami<&str>> = core::hint::black_box(col::cami_unstable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out()))
    .as_vec_ref();
}

//------

library_benchmark_group!(
    name = bench_group;
    config = LibraryBenchmarkConfig::default().env_clear(false);
    compare_by_id = true;
    benchmarks = stable_lexi, stable_cami, unstable_lexi, unstable_cami
);

main!(library_benchmark_groups = bench_group);
