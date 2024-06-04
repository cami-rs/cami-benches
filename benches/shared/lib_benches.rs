// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use cami::prelude::*;
use core::ops::RangeBounds;
use core::{hint, marker::PhantomData, mem, time::Duration};
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
//use ref_cast::RefCast;

extern crate alloc;
use alloc::collections::BTreeSet;

pub fn criterion_config() -> Criterion {
    Criterion::default().warm_up_time(Duration::from_millis(200))
}

// On heap.
pub const MIN_ITEMS: usize = 4; //10;
pub const MAX_ITEMS: usize = 10; //100_000;

#[allow(unused)]
/// On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
/// size may be a few times higher.
pub const MAX_ITEM_LEN: usize = 1_000;

// For purging the L1, L2..., in bytes.
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

/// Shortcut trait, for "output" items based on owned items, but with no specified lifetime.
pub trait Out = Clone + CamiOrd + Ord;
/// Shortcut trait, for "output" items based on owned items, with a lifetime.
pub trait OutLifetimed<'own> = Out + 'own;

/// Collection for "output" items, based on/referencing "owned" items. Used for
/// [OutCollectionIndicator::OutCollectionImpl].
///
/// When implementing [Extend] for this, do implement [Extend::extend_one] and
/// [Extend::extend_reserve], too - even though they do have a default implementation.
///
/// Not extending [core::ops::Index], because [BTreeSet] doesn't extend it either.
pub trait OutCollection<T>: Clone + Extend<T>
where
    T: Out,
{
    // @TODO see if RustDoc/docs.rs/libs.rs generates a correct link for
    // `alloc::collections::BTreeSet``. Otherwise change it to `std::``
    //
    /// For example, `true` for [Vec], `false` for [alloc::collections::BTreeSet].
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool;
    /// If `false`, [OutCollection::sort_unstable] may `panic!` (unsupported).
    const HAS_SORT_UNSTABLE: bool;

    /// Prefer [OutCollection::with_capacity] if possible.
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn clear(&mut self);

    fn len(&self) -> usize;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;
    /// Not required - it may `panic`. @TODO revisit that
    fn into_iter(self) -> impl Iterator<Item = T>;

    /// Like [Iterator::is_sorted]. BUT: For types that maintain/guarantee a sorted order, like
    /// [std::collections::BTreeSet], this must NOT (for example)
    /// - simply return `true`, nor
    /// - just call [std::collections::BTreeSet::iter] -> [Iterator::is_sorted], because that could
    /// be optimized away .
    ///
    /// Instead, it verifies the sorted order. For example: [std::collections::BTreeSet::iter] ->
    /// [core::hint::black_box] -> [Iterator::is_sorted].
    fn is_sorted(&self) -> bool;
    //fn is_sorted_by<F>(&self, compare: F) -> bool where F: FnMut(&T, &T) -> bool;
    fn sort(&mut self);
    /// As per
    /// [`&[]::sort_unstable`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.sort_unstable).
    /// If [OutCollection::HAS_SORT_UNSTABLE] is `false`, this method may `panic!`.
    fn sort_unstable(&mut self);
    // fn sort_by<F>(&mut self, compare: F) where F: FnMut(&T, &T) -> Ordering;
    /// Binary search; return `true` if found an equal item (or key, in case of [alloc::collections::BTreeMap] and friends.)
    fn binary_search(&self, x: &T) -> bool;
    //fn binary_search_by<'this, F>(&'this self, f: F) -> Result<usize, usize> where F: FnMut(&'this T) -> Ordering, T: 'this;
}

pub trait OutCollectionIndicator {
    type OutCollectionImpl<T>: OutCollection<T>
    where
        T: Out;
}

// Vec-based collection
#[derive(Clone)]
#[repr(transparent)]
pub struct OutCollectionVec<T>(pub Vec<T>)
where
    T: Out;

impl<T> Extend<T> for OutCollectionVec<T>
where
    T: Out,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
    fn extend_one(&mut self, item: T) {
        self.0.extend_one(item);
    }
    fn extend_reserve(&mut self, additional: usize) {
        self.0.extend_reserve(additional);
    }
}
impl<T> OutCollection<T> for OutCollectionVec<T>
where
    T: Out,
{
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool = true;
    const HAS_SORT_UNSTABLE: bool = true;

    fn new() -> Self {
        Self(Vec::new())
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    fn clear(&mut self) {
        self.0.clear();
    }

    fn len(&self) -> usize {
        self.0.len()
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.0.iter()
    }
    fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }
    fn is_sorted(&self) -> bool {
        self.0.is_sorted()
    }
    fn sort(&mut self) {
        self.0.sort();
    }
    fn sort_unstable(&mut self) {
        self.0.sort_unstable();
    }
    fn binary_search(&self, x: &T) -> bool {
        self.0.binary_search(x).is_ok()
    }
}

pub struct OutCollectionVecIndicator();
impl OutCollectionIndicator for OutCollectionVecIndicator {
    type OutCollectionImpl<T> = OutCollectionVec<T> where T: Out;
}
// End of: Vec-based collection

// BTreeSet-based collection:
#[derive(Clone)]
#[repr(transparent)]
pub struct OutCollectionBTreeSet<T>(pub BTreeSet<T>)
where
    T: Out;

impl<T> Extend<T> for OutCollectionBTreeSet<T>
where
    T: Out,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
    fn extend_one(&mut self, item: T) {
        self.0.extend_one(item);
    }
    fn extend_reserve(&mut self, additional: usize) {
        self.0.extend_reserve(additional);
    }
}
impl<T> OutCollection<T> for OutCollectionBTreeSet<T>
where
    T: Out,
{
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool = false;
    const HAS_SORT_UNSTABLE: bool = false;

    fn new() -> Self {
        Self(BTreeSet::new())
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(BTreeSet::new())
    }
    fn clear(&mut self) {
        self.0.clear();
    }

    fn len(&self) -> usize {
        self.0.len()
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.0.iter()
    }
    fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }
    fn is_sorted(&self) -> bool {
        let iter = self.0.iter();
        hint::black_box(iter).is_sorted()
    }
    fn sort(&mut self) {}
    fn sort_unstable(&mut self) {
        unreachable!();
    }
    fn binary_search(&self, x: &T) -> bool {
        self.0.get(x).is_some()
    }
}

pub struct OutCollectionBTreeSetIndicator();
impl OutCollectionIndicator for OutCollectionBTreeSetIndicator {
    type OutCollectionImpl<T> = OutCollectionBTreeSet<T> where T: Out;
}
// End of: BTreeSet-based collection

type OutCollRetrieverPerItem<OutCollectionIndicatorImpl, T> =
    <OutCollectionIndicatorImpl as OutCollectionIndicator>::OutCollectionImpl<T>;

type OutRetriever<'own, OutIndicatorIndicatorImpl, Sub> =
    <<OutIndicatorIndicatorImpl as OutIndicatorIndicator>::OutIndicatorImpl<
        'own,
        Sub,
    > as OutIndicator<'own, Sub>>::OutLifetimedImpl;

type OutCollRetriever<'own, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, Sub> =
    OutCollRetrieverPerItem<
        OutCollectionIndicatorImpl,
        OutRetriever<'own, OutIndicatorIndicatorImpl, Sub>,
    >;
// Previous `TransRef` is at
// https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/DropCk.20.26.20GAT.20.28Generic.20Associative.20Types.29

//-----
/// `Sub` means sub-item/component of [Out].
pub trait OutIndicator<'own, Sub>
where
    Sub: OutLifetimed<'own>,
{
    type OutLifetimedImpl: OutLifetimed<'own> + 'own;
}
pub trait OutIndicatorIndicator {
    type OutIndicatorImpl<'own, Sub>: OutIndicator<'own, Sub>
    where
        Sub: OutLifetimed<'own>;
}
pub struct OutIndicatorNonRef<Sub>(PhantomData<Sub>);
impl<'own, OutItem> OutIndicator<'own, OutItem> for OutIndicatorNonRef<OutItem>
where
    OutItem: OutLifetimed<'own>,
{
    type OutLifetimedImpl = OutItem;
}
pub struct OutIndicatorNonRefIndicator();
impl OutIndicatorIndicator for OutIndicatorNonRefIndicator {
    type OutIndicatorImpl<'own, T> = OutIndicatorNonRef<T> where T: OutLifetimed<'own>;
}
pub struct OutIndicatorSlice<Sub>(PhantomData<Sub>);
impl<'own, Sub> OutIndicator<'own, Sub> for OutIndicatorSlice<Sub>
where
    Sub: OutLifetimed<'own>,
{
    type OutLifetimedImpl = &'own [Sub];
}
pub struct OutIndicatorSliceIndicator();
impl OutIndicatorIndicator for OutIndicatorSliceIndicator {
    type OutIndicatorImpl<'own, T> = OutIndicatorSlice<T> where T: OutLifetimed<'own>;
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
    c: &mut Criterion,
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
        c,
        rnd,
        group_name,
        id_state,
        generate_id_postfix,
        generate_out_item,
    );
}

/// This removes any extra equal items from `own_items` if the indicated [OutCollection] has
/// [OutCollection::ALLOWS_MULTIPLE_EQUAL_ITEMS] being `false`. No guarantee as to which one of any
/// two or more equal items will stay.
pub fn bench_vec_sort_bin_search_own_items<
    OwnType: Ord,
    SubType: Out,
    OutIndicatorIndicatorImpl: OutIndicatorIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
    Rnd: Random,
    IdState,
>(
    own_items: Vec<OwnType>,
    c: &mut Criterion,
    rnd: &mut Rnd,
    group_name: impl Into<String>,
    id_state: &IdState,
    generate_id_postfix: impl Fn(&IdState) -> String,
    generate_out_item: impl Fn(&OwnType) -> OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
) {
    bench_vec_sort_bin_search_lifetimed::<
        '_,
        OwnType,
        SubType,
        OutRetriever<'_, OutIndicatorIndicatorImpl, SubType>,
        OutCollRetriever<'_, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, SubType>,
        Rnd,
        IdState,
    >(
        own_items,
        c,
        rnd,
        group_name,
        id_state,
        generate_id_postfix,
        generate_out_item,
    );
}

pub fn bench_vec_sort_bin_search_lifetimed<
    'own,
    OwnType: Ord + 'own,
    SubType: Out,
    // Two "retrieved" types:
    OutType: Out,
    OutCollectionType: OutCollection<OutType>,
    // No need for type indicators here.
    Rnd: Random,
    IdState,
>(
    mut own_items: Vec<OwnType>,
    c: &mut Criterion,
    rnd: &mut Rnd,
    group_name: impl Into<String>,
    id_state: &IdState,
    generate_id_postfix: impl Fn(&IdState) -> String,
    generate_out_item: impl Fn(&'own OwnType) -> OutType,
) {
    let mut group = c.benchmark_group(group_name);

    if !OutCollectionType::ALLOWS_MULTIPLE_EQUAL_ITEMS {
        let mut set = BTreeSet::<OwnType>::new();
        set.extend(own_items.into_iter());
        own_items = Vec::<OwnType>::with_capacity(set.len());
        own_items.extend(set.into_iter());
    }

    {
        let mut unsorted_items = OutCollectionType::with_capacity(1);

        // let unsorted_items = unsorted_items; // Prevent mutation by mistake.

        let id_string = format!(
            "{} items, each len max {MAX_ITEM_LEN}.{}",
            own_items.len(),
            generate_id_postfix(id_state)
        );
        //#[cfg(do_later)]
        if false {
            let mut sorted_lexi = OutCollectionType::with_capacity(1);
            //let mut sorted_lexi = unsorted_items.clone();
            group.bench_with_input(
                BenchmarkId::new("std sort lexi.          ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        //sorted_lexi = hint::black_box(unsorted_items.clone());
                        // @TODO ^^^--> .clone()  \----> change to:
                        //
                        // .sorted_lexi.extend( it().map(|it_ref| it_ref.clone()))
                        sorted_lexi.clear();
                        sorted_lexi.extend(unsorted_items.iter().cloned());

                        //sorted_lexi.sort_by(<OutItemIndicatorImpl as OutItemIndicator>::OutItemLifetimedImpl::cmp);
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
        //#[cfg(do_later)]
        {
            purge_cache(rnd);
            #[cfg(not(feature = "transmute"))]
            let unsorted_items = {
                let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
                unsorted_items_cami
                    .extend(unsorted_items.iter().map(|v| Cami::<TO>::new(v.clone())));
                unsorted_items_cami
            };

            let mut sorted_non_lexi = Vec::new();
            group.bench_with_input(
                BenchmarkId::new("std sort non-lexi.      ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        #[cfg(feature = "transmute")]
                        let _ = {
                            // @TODO replace .clone() by: Vec::with_capacity(), .iter() -> extend -> .into_vec_cami()
                            let unsorted_items = (*unsorted_items).clone();

                            // @TODO TODO
                            //sorted_non_lexi = hint::black_box(unsorted_items).into_vec().into_vec_cami();
                        };
                        #[cfg(not(feature = "transmute"))]
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
                        // The following `unsorted_items.into_iter()` is cheap (no consuming of any `Vec`), because `unsorted_items`` is a reference to a Vec.
                        for item in hint::black_box(unsorted_items.iter()) {
                            #[cfg(feature = "transmute")]
                            let _ = {
                                hint::black_box(sorted.binary_search(item.into_ref_cami()))
                                    .unwrap();
                            };
                            #[cfg(not(feature = "transmute"))]
                            let _ = {
                                hint::black_box(sorted.binary_search(item)).unwrap();
                            };
                        }
                    })
                },
            );
        }
    }
    group.finish();
}
