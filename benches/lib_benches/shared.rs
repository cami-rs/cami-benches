// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use super::outish::*;
use cami::prelude::*;
use core::hint;
use core::ops::RangeBounds;
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
//use ref_cast::RefCast;

use alloc::collections::BTreeSet;

/// Min number of test items.
pub const MIN_ITEMS: usize = 4;
/// Max. number of test items.
pub const MAX_ITEMS: usize = 10;

#[allow(unused)]
/// On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
/// size may be a few times higher.
pub const MAX_ITEM_LEN: usize = 1_000;

/// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

pub trait Random {
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8;
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize;
}

impl Random for Rng {
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        Rng::usize(self, range)
    }
}

pub fn purge_cache<RND: Random>(rng: &mut RND) {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(rng.u8(..));
    }
    hint::black_box(vec);
}
//------

/// `OwnType` needs to be [Ord] only if `generate_own_item` can generate (some) equal items AND if
/// the indicated [OutCollection] has [OutCollection::ALLOWS_MULTIPLE_EQUAL_ITEMS] being `false`.
pub fn bench_vec_sort_bin_search<
    OwnType: Ord,
    SubType: Out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
    Rnd: Random,
    IdState,
>(
    critty: &mut Criterion,
    rnd: &mut Rnd,
    group_name: impl Into<String>,
    id_state: &mut IdState,
    generate_id_postfix: impl Fn(&IdState) -> String,
    generate_own_item: impl Fn(&mut Rnd, &mut IdState) -> OwnType,
    generate_out_item: impl Fn(&OwnType) -> OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
) {
    let num_items = rnd.usize(MIN_ITEMS..MAX_ITEMS);

    let mut own_items = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        let item = generate_own_item(rnd, id_state);
        own_items.push(item);
    }

    bench_vec_sort_bin_search_own_items::<
        OwnType,
        SubType,
        OutIndicatorIndicatorImpl,
        OutCollectionIndicatorImpl,
        Rnd,
        IdState,
    >(
        own_items,
        critty,
        rnd,
        group_name,
        id_state,
        generate_id_postfix,
        generate_out_item,
    );
}

/// This removes any extra equal items from `own_items` if the indicated [OutCollection] has
/// [OutCollection::ALLOWS_MULTIPLE_EQUAL_ITEMS] being `false`. No guarantee as to which one of any
/// two (or more) equal items will stay.
pub fn bench_vec_sort_bin_search_own_items<
    OwnType: Ord,
    SubType: Out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
    Rnd: Random,
    IdState,
>(
    mut own_items: Vec<OwnType>,
    critty: &mut Criterion,
    rnd: &mut Rnd,
    group_name: impl Into<String>,
    id_state: &IdState,
    generate_id_postfix: impl Fn(&IdState) -> String,
    generate_out_item: impl Fn(&OwnType) -> OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
) {
    bench_vec_sort_bin_search_ref_possibly_duplicates::<
        OwnType,
        OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'_, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
        Rnd,
        IdState,
    >(
        &mut own_items,
        critty,
        rnd,
        group_name,
        id_state,
        generate_id_postfix,
        generate_out_item,
    );
}

pub fn bench_vec_sort_bin_search_ref_possibly_duplicates<
    'own,
    OwnType: Ord + 'own,
    // No need for SubType from this level deeper.
    //
    // Two "retrieved" types:
    OutType: Out + 'own,
    OutCollectionType: OutCollection<'own, OutType> + 'own,
    // No need for type indicators from this level deeper.
    Rnd: Random,
    IdState,
>(
    own_items: &'own mut Vec<OwnType>,
    critty: &mut Criterion,
    rnd: &mut Rnd,
    group_name: impl Into<String>,
    id_state: &IdState,
    generate_id_postfix: impl Fn(&IdState) -> String,
    generate_out_item: impl Fn(&'own OwnType) -> OutType,
) {
    if !OutCollectionType::ALLOWS_MULTIPLE_EQUAL_ITEMS {
        // Remove duplicates. Yes, the result may have fewer items than planned/configured.
        let mut set = BTreeSet::<OwnType>::new();
        set.extend(own_items.drain(..));
        own_items.extend(set.into_iter());
    }

    bench_vec_sort_bin_search_ref::<OwnType, OutType, OutCollectionType, Rnd, IdState>(
        own_items,
        critty,
        rnd,
        group_name,
        id_state,
        generate_id_postfix,
        generate_out_item,
    );
}

pub fn bench_vec_sort_bin_search_ref<
    'own,
    OwnType: Ord + 'own,
    OutType: Out + 'own,
    OutCollectionType: OutCollection<'own, OutType> + 'own,
    Rnd: Random,
    IdState,
>(
    own_items: &'own Vec<OwnType>,
    critty: &mut Criterion,
    rnd: &mut Rnd,
    group_name: impl Into<String>,
    id_state: &IdState,
    generate_id_postfix: impl Fn(&IdState) -> String,
    // @TODO _generate_out_item
    _generate_out_item: impl Fn(&'own OwnType) -> OutType,
) {
    let mut group = critty.benchmark_group(group_name);
    {
        let unsorted_items = {
            let unsorted_items = OutCollectionType::with_capacity(own_items.len());
            //unsorted_items.extend(own_items.iter().map(generate_out_item)); ^^
            unsorted_items
        };

        own_items.iter().for_each(|_rf| { //@TODO?
             //generate_out_item(rf); ^^^
        });

        fn consume_own_ref<'ownsh, OwnishType: Ord + 'ownsh>(_o: &'ownsh OwnishType) {}
        own_items.iter().for_each(|rf| {
            consume_own_ref(rf);
        });

        own_items.iter().for_each(|rf| {
            core::hint::black_box(rf);
        });

        let mut _refs = vec![];
        _refs.extend(own_items.iter());

        let mut refs = vec![];
        refs.extend(own_items.iter().map(core::convert::identity));

        let id_string = format!(
            "{} items, each len max {MAX_ITEM_LEN}.{}",
            own_items.len(),
            generate_id_postfix(id_state)
        );
        if false {
            let mut sorted_lexi = OutCollectionType::with_capacity(1);
            group.bench_with_input(
                BenchmarkId::new("std sort lexi.          ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        //sorted_lexi = hint::black_box(unsorted_items.clone()); @TODO ^^^-->
                        // .clone()  \----> change to:
                        //
                        // .sorted_lexi.extend( it().map(|it_ref| it_ref.clone()))
                        sorted_lexi.clear();
                        sorted_lexi.extend(unsorted_items.iter().cloned());

                        //sorted_lexi.sort_by(<OutItemIndicatorImpl as
                        //OutItemIndicator>::OutItemLifetimedImpl::cmp);
                        sorted_lexi.sort();
                    })
                },
            );
            purge_cache(rnd);
            group.bench_with_input(
                BenchmarkId::new("std bin search (lexi)   ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        let sorted = hint::black_box(&sorted_lexi);
                        for item in hint::black_box(unsorted_items.iter()) {
                            assert!(hint::black_box(sorted.binary_search(&item)));
                        }
                    })
                },
            );
        }
        {
            purge_cache(rnd);
            // @TODO cfg
            //
            //#[cfg(not(feature = "transmute"))]
            let unsorted_items = {
                let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
                unsorted_items_cami.extend(
                    unsorted_items
                        .iter()
                        .map(|v| Cami::<OutType>::new(v.clone())),
                );
                unsorted_items_cami
            };

            let mut sorted_non_lexi = Vec::new();
            group.bench_with_input(
                BenchmarkId::new("std sort non-lexi.      ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        // @TODO cfg
                        //
                        /*
                        #[cfg(feature = "transmute")]
                        let _ = {
                            // @TODO replace .clone() by: Vec::with_capacity(), .iter() -> extend ->
                            // .into_vec_cami()
                            let unsorted_items = (*unsorted_items).clone();

                            // @TODO TODO sorted_non_lexi =
                            //hint::black_box(unsorted_items).into_vec().into_vec_cami();
                        };
                        */
                        // @TODO cfg
                        //
                        // #[cfg(not(feature = "transmute"))]
                        let _ = {
                            sorted_non_lexi = hint::black_box(unsorted_items.clone());
                        };
                        sorted_non_lexi.sort();
                    })
                },
            );
            purge_cache(rnd);
            group.bench_with_input(
                BenchmarkId::new("std bin search (non-lexi)", id_string),
                hint::black_box(&unsorted_items),
                //hint::black_box( unsorted_items.into_ref_vec_cami() ),
                |b, unsorted_items| {
                    b.iter(|| {
                        let sorted = hint::black_box(&sorted_non_lexi);
                        for item in hint::black_box(unsorted_items.iter()) {
                            // @TODO cfg
                            //
                            /*#[cfg(feature = "transmute")]
                            let _ = {
                                hint::black_box(sorted.binary_search(item.into_ref_cami()))
                                    .unwrap();
                            };*/
                            // @TODO cfg
                            //
                            //#[cfg(not(feature = "transmute"))]
                            let _ = {
                                hint::black_box(sorted.binary_search(item)).unwrap();
                            };
                        }
                    })
                },
            );
        }
        let _refs = refs;
    }
    group.finish();
}
