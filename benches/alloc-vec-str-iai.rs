#![feature(extend_one)]
#![feature(is_sorted)]
#![feature(thread_id_value)]
#![feature(trait_alias)]

use cami::prelude::*;
use cami_benches::data::{self, Data, OwnAndOut};
use cami_benches::outish::{OutCollectionVec, OutCollectionVecIndicator, OutIndicatorStrIndicator};
use cami_benches::{col, shared_iai};
use core::iter;
use fastrand::Rng;
use iai_callgrind::{library_benchmark, library_benchmark_group, main, LibraryBenchmarkConfig};

type OutType = &'static str;
type OutTypeRef = &'static [OutType];

fn out() -> OutTypeRef {
    let own_and_out = OwnAndOut::new(|rnd: &mut Rng| rnd.string(), |string| &string[..], true);
    //data::purge_cache();
    own_and_out.out
}

//------

#[library_benchmark]
#[bench::stable(out())]
fn stable_lexi(out: OutTypeRef) -> OutCollectionVec<'static, &'static str> {
    core::hint::black_box(col::lexi_stable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out))
}

#[library_benchmark]
#[bench::stable(out())]
fn stable_cami(out: OutTypeRef) -> OutCollectionVec<'static, Cami<&'static str>> {
    core::hint::black_box(col::cami_stable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out))
}
//------

#[library_benchmark]
#[bench::unstable(out())]
fn unstable_lexi(out: OutTypeRef) {
    let _result: &Vec<&str> = core::hint::black_box(col::lexi_unstable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out))
    .as_vec_ref();
}

#[library_benchmark]
#[bench::unstable(out())]
fn unstable_cami(out: OutTypeRef) {
    let _result: &Vec<Cami<&str>> = core::hint::black_box(col::cami_unstable::<
        &str,
        OutIndicatorStrIndicator,
        OutCollectionVecIndicator,
    >(out))
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
