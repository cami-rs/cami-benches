// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use super::outish::*;
use alloc::collections::BTreeSet;
use cami::prelude::*;
use core::hint;
use core::ops::RangeBounds;
use fastrand::Rng;
//use ref_cast::RefCast;

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

/// We create one instance per set of compared benchmarks. We don't re-use the same instance for all
/// benchmarks, because we'd need mutable access to such instance, and that's tricky with
/// `iai-callgrind`'s, `Criterion`'s or other harness's macros. That would prevent benchmarking in
/// parallel.
///
/// Therefore we require the user to provide a seed.
pub trait Random {
    fn with_seed_dec(seed: &str) -> Self;
    fn with_seed_hex(seed: &str) -> Self;

    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8;
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize;
}

impl Random for Rng {
    fn with_seed_dec(seed: u64) -> Self {
        //Rng::with_seed(seed)
        panic!()
    }
    fn with_seed_hex(seed: u64) -> Self {
        //Rng::with_seed(seed)
        panic!()
    }

    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        Rng::usize(self, range)
    }
} //------

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
//-------

/// Stores (static, leaked) "own" & "out" data, where "out" potentially borrows from "own". When an
/// instance (of this struct) is stored in a (`static`) [once_cell::sync::OnceCell], these two
/// references add one level of indirection compare to having two separate (`static`)
/// [once_cell::sync::OnceCell] instances (which would mean more synchronization). But we want
/// simplicity.
pub struct DataOwnAndOut<OwnType: 'static, OutType: Out + 'static> {
    pub own: &'static [OwnType],
    /// Unsorted.
    pub out: &'static [OutType],
}

impl<OwnType: 'static, OutType: Out + 'static> DataOwnAndOut<OwnType, OutType> {
    pub fn new(
        generate_own_item: impl Fn(&mut RndChoice) -> OwnType,
        generate_out_item: impl Fn(&'static OwnType) -> OutType,
        allows_multiple_equal_items: bool,
    ) -> Self {
        let own = data_own(generate_own_item).leak();

        let mut out: Vec<OutType> = Vec::<OutType>::with_capacity(own.len());
        out.extend(own.iter().map(generate_out_item));

        if !allows_multiple_equal_items {
            let len_including_duplicates = out.len();
            // Remove duplicates. Yes, the result may have fewer items than planned/configured.
            let mut set = BTreeSet::<OutType>::new();
            set.extend(out.drain(..));
            out.extend(set.into_iter());

            if out.len() < MIN_ITEMS_AFTER_REMOVING_DUPLICATES {
                panic!("Benchmarking requires min. of {MIN_ITEMS_AFTER_REMOVING_DUPLICATES} unduplicated items. There was {} 'own' items, and {len_including_duplicates} generated ('out'). But, after removing duplicates, there was only {} items left! Re-run, change the limits, or investigate.", own.len(), out.len());
            }
        }
        let out = out.leak();
        Self { own, out }
    }
}

/// Collect and sort.
pub fn lexi_stable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    lexi_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, true)
}
/// Collect and sort unstable. If the collection doesn't support unstable sort, this may [panic].
pub fn lexi_unstable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    lexi_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, false)
}
/// Collect. If the collection doesn't keep sorted order, then this does NOT sort.
pub fn lexi_indicated<
    'out,
    OutType: Out + 'out,
    OutCollectionLexi: OutCollection<'out, OutType>,
>(
    out: &'out [OutType],
    stable_sort: bool,
) -> OutCollectionLexi {
    let mut col = OutCollectionLexi::with_capacity(out.len());
    col.extend(out.iter().cloned());
    if stable_sort {
        col.sort();
    } else {
        col.sort_unstable();
    }
    col
}

/// Collect [Cami] wrapers around items and sort.
pub fn cami_stable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    cami_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, true)
}
/// Collect and sort unstable. If the collection doesn't support unstable sort, this may [panic].
pub fn cami_unstable<
    'out,
    SubType: Out + 'out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
>(
    out: &'out [OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>],
) -> OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType> {
    cami_indicated::<
        OutRetriever<'out, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetrieverCami<'out, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
    >(out, false)
}
/// Collect. If the collection doesn't keep sorted order, then this does NOT sort.
pub fn cami_indicated<
    'out,
    OutType: Out + 'out,
    OutCollectionCami: OutCollection<'out, Cami<OutType>>,
>(
    out: &'out [OutType],
    stable_sort: bool,
) -> OutCollectionCami {
    let mut col = OutCollectionCami::with_capacity(out.len());
    col.extend(out.iter().cloned().map(Cami::new));
    if stable_sort {
        col.sort();
    } else {
        col.sort_unstable();
    }
    col
}
