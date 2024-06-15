// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use super::outish::*;
use cami::prelude::*;
use core::marker::PhantomData;
use core::ops::RangeBounds;
use fastrand::Rng;
use std::hint;
//use ref_cast::RefCast;

use alloc::collections::BTreeSet;

extern crate alloc;

/// If calling [data_out] with [[OutCollection] that has[OutCollection::ALLOWS_MULTIPLE_EQUAL_ITEMS]
/// set to `true`, then [MIN_ITEMS_AFTER_REMOVING_DUPLICATES] is the minimum number of items
/// required for benchmarking to continue. Otherwise we get a [panic].
pub const MIN_ITEMS_AFTER_REMOVING_DUPLICATES: usize = 4;

/// Min number of test items.
pub const MIN_ITEMS: usize = 40;
/// Max. number of test items.
pub const MAX_ITEMS: usize = 10000;

#[allow(unused)]
/// On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
/// size may be a few times higher.
pub const MAX_ITEM_LEN: usize = 1_000;

/// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

pub trait Random {
    /// We create one instance per set of compared benchmarks. We don't re-use the same instance for
    /// all benchmarks, because we'd need mutable access to such instance, and that's tricky with
    /// `iai-callgrind`'s, `Criterion`'s or other harness's macros. That would prevent benchmarking
    /// in parallel.
    fn new() -> Self;
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8;
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize;
}

impl Random for Rng {
    fn new() -> Self {
        Rng::new()
    }
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        Rng::usize(self, range)
    }
}

pub fn purge_cache() {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(core::hint::black_box(1));
    }
    core::hint::black_box(vec);
}
//------

/// Some of the fields are equal to results of operations that themselves get benchmarked, too.
/// However, none of these fields comes from a result of any benchmark, because
/// - that would make benchmark files ugly, and
/// - there is no guaranteed order of benchmarks, and
/// - we'd have to turn off & on the measuring/capturing, and
/// - we'd need a separate `static mut`, or `static ... : OnceCell<...>` variable for each...
///
/// So, all of these fields (except for `unsorted`) are "duplicates" - and that's OK.
pub struct DataOut<
    'own,
    OutType: Out + 'own,
    OutCollectionClassic: OutCollection<'own, OutType>,
    OutCollectionCami: OutCollection<'own, Cami<OutType>>,
> {
    pub unsorted_vec_classic: Vec<OutType>,

    pub unsorted_col_classic: OutCollectionClassic,
    /// "Classic" ordering (lexicographic)
    pub sorted_col_classic: OutCollectionClassic,

    pub unsorted_col_cami: OutCollectionCami,
    /// Cami ordering (potentially non-lexicographic)
    pub sorted_col_cami: OutCollectionCami,

    // @TODO remove:
    pub unsorted_vec_cami: Vec<Cami<OutType>>,
    // @TODO remove:
    /// Cami sorting (potentially non-lexicographic)
    pub sorted_vec_cami: Vec<Cami<OutType>>,
    _own: PhantomData<&'own ()>,
}
//------

pub fn bench<
    OwnType,
    SubType: Out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    mut own_items: Vec<OwnType>,
    generate_out_item: impl Fn(&OwnType) -> OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
) {
    bench_with_col_types::<
        OwnType,
        OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'_, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetrieverCami<'_, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(&mut own_items, generate_out_item);
}

fn data_own_for_rnd<OwnType, Rnd: Random>(
    generate_own_item: impl Fn(&mut Rnd) -> OwnType,
) -> Vec<OwnType> {
    let mut rnd = Rnd::new();
    let num_items = rnd.usize(MIN_ITEMS..MAX_ITEMS);
    let mut own_items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        let item = generate_own_item(&mut rnd);
        own_items.push(item);
    }
    own_items
}

#[cfg(feature = "fastrand")]
type RndChoice = Rng;
#[cfg(not(feature = "fastrand"))]
compile_error!("Currently we require 'fastrand' feature.");

pub fn data_own<OwnType>(generate_own_item: impl Fn(&mut RndChoice) -> OwnType) -> Vec<OwnType> {
    data_own_for_rnd::<OwnType, RndChoice>(generate_own_item)
}

/// This removes any extra equal `OwnType` items (duplicates), if the indicated [OutCollection] has
/// [OutCollection::ALLOWS_MULTIPLE_EQUAL_ITEMS] being `false`. No guarantee as to which one of any
/// two (or more) equal items will stay.
pub fn data_out<
    'own,
    OwnType: 'own,
    OutType: Out + 'own,
    OutCollectionType: OutCollection<'own, OutType>,
    OutCollectionCami: OutCollection<'own, Cami<OutType>>,
>(
    own_items: &'own Vec<OwnType>,
    generate_out_item: impl Fn(&'own OwnType) -> OutType,
) -> DataOut<'own, OutType, OutCollectionType, OutCollectionCami> {
    let unsorted_vec_classic = {
        let mut unsorted = Vec::<OutType>::with_capacity(own_items.len());
        unsorted.extend(own_items.iter().map(generate_out_item));

        if !OutCollectionType::ALLOWS_MULTIPLE_EQUAL_ITEMS {
            let unsorted_with_duplicates_len = unsorted.len();
            // Remove duplicates. Yes, the result may have fewer items than planned/configured.
            let mut set = BTreeSet::<OutType>::new();
            set.extend(unsorted.drain(..));
            unsorted.extend(set.into_iter());
            if unsorted.len() < MIN_ITEMS_AFTER_REMOVING_DUPLICATES {
                panic!("Benchmarking requires min. of {MIN_ITEMS_AFTER_REMOVING_DUPLICATES} unduplicated items. There was {own_items.len()} 'own' items, and {unsorted_with_duplicates_len} generated ('out'). But, after removing duplicates, there was only {unsorted.len()} items left! Re-run, change the limits, or investigate.");
            }
        }
        unsorted
    };

    let unsorted_col_classic = {
        let mut unsorted = OutCollectionType::with_capacity(unsorted_vec_classic.len());
        unsorted.extend(unsorted_vec_classic.iter().cloned());
        unsorted
    };

    //@TODO into a separate function each

    let sorted_col_classic = {
        let mut sorted = unsorted_col_classic.clone();
        sorted.sort();
        sorted
    };

    let unsorted_vec_cami: Vec<Cami<OutType>> = {
        let mut unsorted_cami = Vec::with_capacity(unsorted_vec_classic.len());
        unsorted_cami.extend(
            unsorted_vec_classic
                .iter()
                .cloned()
                .map(Cami::<OutType>::new),
        );
        unsorted_cami
    };

    let sorted_vec_cami: Vec<Cami<OutType>> = {
        let mut sorted = unsorted_vec_cami.clone();
        sorted.sort();
        sorted
    };

    let unsorted_col_cami = {
        let mut unsorted = OutCollectionCami::with_capacity(unsorted_vec_classic.len());
        unsorted.extend(unsorted_vec_cami.iter().cloned());
        unsorted
    };

    let sorted_col_cami = {
        let mut sorted = unsorted_col_cami.clone();
        sorted.sort();
        sorted
    };

    DataOut {
        unsorted_vec_classic,

        unsorted_col_classic,
        sorted_col_classic,

        unsorted_col_cami,
        sorted_col_cami,

        unsorted_vec_cami,
        sorted_vec_cami,
        _own: PhantomData,
    }
}

pub fn bench_with_col_types<
    'own,
    OwnType: 'own,
    OutType: Out + 'own,
    OutCollectionType: OutCollection<'own, OutType>,
    OutCollectionTypeCami: OutCollection<'own, Cami<OutType>>,
>(
    own_items: &'own Vec<OwnType>,
    generate_out_item: impl Fn(&'own OwnType) -> OutType,
) {
    let unsorted_items = {
        let mut unsorted_items = OutCollectionType::with_capacity(own_items.len());
        unsorted_items.extend(own_items.iter().map(generate_out_item));
        unsorted_items
    };

    if false {
        let sorted_lexi =
        // @TODO bench
        {
            let mut sorted_lexi = OutCollectionType::with_capacity(1);
            // "std sort lexi.          "
            let unsorted_items = &unsorted_items;
            //sorted_lexi = hint::black_box(unsorted_items.clone()); @TODO ^^^--> .clone()  \---->
            // change to:
            //
            // .sorted_lexi.extend( it().map(|it_ref| it_ref.clone()))
            sorted_lexi.clear();
            sorted_lexi.extend(unsorted_items.iter().cloned());

            //sorted_lexi.sort_by(<OutItemIndicatorImpl as
            //OutItemIndicator>::OutItemLifetimedImpl::cmp);
            sorted_lexi.sort();
            sorted_lexi
        };
        purge_cache();

        {
            // "std bin search (lexi)   "
            let unsorted_items = &unsorted_items;
            let sorted = hint::black_box(&sorted_lexi);
            for item in hint::black_box(unsorted_items.iter()) {
                assert!(hint::black_box(sorted.binary_search(&item)));
            }
        }
        purge_cache();

        {
            // If we can't transmute, then we clone().
            //
            // @TODO cfg
            //
            //#[cfg(not(feature = "transmute"))]
            let unsorted_items: Vec<Cami<OutType>> = {
                let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
                unsorted_items_cami.extend(
                    unsorted_items
                        .iter()
                        .map(|v| Cami::<OutType>::new(v.clone())),
                );
                unsorted_items_cami
            };

            let mut sorted_non_lexi = Vec::new();
            {
                // "std sort non-lexi.      "
                let unsorted_items = &unsorted_items;
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
            }
            purge_cache();

            {
                // "std bin search (non-lexi)"
                let unsorted_items = &unsorted_items;
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
            }
        }
    }
}
