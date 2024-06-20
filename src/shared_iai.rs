// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use super::outish::*;
use alloc::collections::BTreeSet;
use cami::prelude::*;
use core::marker::PhantomData;
use core::ops::RangeBounds;
use fastrand::Rng;
use std::hint;
use std::str::FromStr;

extern crate alloc;

//------

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

pub type DataOutIndicated<
    SubType: Out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
> = DataOut<
    'static,
    OutRetriever<'static, OutIndicatorIndicatorImpl, SubType>,
    OutCollRetriever<'static, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    OutCollRetrieverCami<'static, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
>;

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

pub fn data_out_indicated<
    OwnType,
    SubType: Out + 'static,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    own_items: &'static Vec<OwnType>,
    generate_out_item: impl Fn(&'static OwnType) -> OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
) -> DataOutIndicated<SubType, OutIndicatorIndicatorImpl, OutCollectionIndicatorImpl> {
    data_out(own_items, generate_out_item)
}
