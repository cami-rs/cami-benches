use alloc::collections::BTreeSet;
use cami::CamiOrd;
use core::marker::PhantomData;

extern crate alloc;

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
pub trait OutCollection<'out, T>: Clone + Extend<T>
where
    T: Out + 'out,
{
    // @TODO see if RustDoc/docs.rs/libs.rs generates a correct link for
    // `alloc::collections::BTreeSet``. Otherwise change it to `std::``
    //
    /// For example, `true` for [Vec], `false` for [alloc::collections::BTreeSet].
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool;
    /// If `false`, [OutCollection::sort_unstable] may `panic!` (unsupported).
    const HAS_SORT_UNSTABLE: bool;
    /// If `false`, [OutCollection::sort] may `panic!` (unsupported). Normally `true` in development
    /// with `std` or `alloc`.
    const HAS_SORT: bool;

    /// Prefer [OutCollection::with_capacity] if possible.
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn clear(&mut self);

    fn len(&self) -> usize;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    /// Like [Iterator::is_sorted]. BUT: For types that maintain/guarantee a sorted order, like
    /// [std::collections::BTreeSet], this must NOT (for example)
    /// - simply return `true`, nor
    /// - just call [std::collections::BTreeSet::iter] -> [Iterator::is_sorted], because that could
    /// be optimized away .
    ///
    /// Instead, it verifies the sorted order. For example: [std::collections::BTreeSet::iter] ->
    /// [core::hint::black_box] -> [Iterator::is_sorted].
    fn is_sorted(&self) -> bool;
    fn sort(&mut self);
    /// As per
    /// [`&[]::sort_unstable`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.sort_unstable).
    /// If [OutCollection::HAS_SORT_UNSTABLE] is `false`, this method may `panic!`.
    fn sort_unstable(&mut self);
    /// Binary search; return `true` if found an equal item (or key, in case of
    /// [alloc::collections::BTreeMap] and friends.)
    fn binary_search(&self, x: &T) -> bool;
}

pub trait OutCollectionIndicator {
    type OutCollectionImpl<'own, T>: OutCollection<'own, T>
    where
        T: Out + 'own;
}

/// `Vec`-based collection
#[derive(Clone)]
#[repr(transparent)]
pub struct OutCollectionVec<'out, T>(pub Vec<T>, PhantomData<&'out ()>)
where
    T: Out + 'out;

impl<'own, T> Extend<T> for OutCollectionVec<'own, T>
where
    T: Out + 'own,
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
impl<'own, T> OutCollection<'own, T> for OutCollectionVec<'own, T>
where
    T: Out + 'own,
{
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool = true;
    const HAS_SORT_UNSTABLE: bool = true;
    const HAS_SORT: bool = true;

    fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity), PhantomData)
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
    type OutCollectionImpl<'own, T> = OutCollectionVec<'own, T> where T: Out + 'own;
}
// End of: Vec-based collection

/// `BTreeSet`-based collection:
#[derive(Clone)]
#[repr(transparent)]
pub struct OutCollectionBTreeSet<'own, T>(pub BTreeSet<T>, PhantomData<&'own ()>)
where
    T: Out + 'own;

impl<'own, T> Extend<T> for OutCollectionBTreeSet<'own, T>
where
    T: Out + 'own,
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
impl<'own, T> OutCollection<'own, T> for OutCollectionBTreeSet<'own, T>
where
    T: Out + 'own,
{
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool = false;
    const HAS_SORT_UNSTABLE: bool = false;
    const HAS_SORT: bool = true;

    fn new() -> Self {
        Self(BTreeSet::new(), PhantomData)
    }
    fn with_capacity(_capacity: usize) -> Self {
        Self(BTreeSet::new(), PhantomData)
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
    fn is_sorted(&self) -> bool {
        let iter = self.0.iter();
        core::hint::black_box(iter).is_sorted()
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
    type OutCollectionImpl<'own, T> = OutCollectionBTreeSet<'own, T> where T: Out + 'own;
}
// End of: BTreeSet-based collection

/// mut slice-based collection.
///
/// This is for benchmarking `cami` without  `alloc` and `std` features, that is, for `no_std` & no
/// `alloc`.
///
/// The actual benchmarking collection does use `Vec`. But, when it invokes `cami`, it does so by
/// passing only a slice (mutable, where appropriate).
#[derive(Clone)]
#[repr(transparent)]
pub struct OutCollectionSlice<'own, T>(pub Vec<T>, PhantomData<&'own ()>)
where
    T: Out + 'own;

impl<'own, T> Extend<T> for OutCollectionSlice<'own, T>
where
    T: Out + 'own,
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
impl<'own, T> OutCollectionSlice<'own, T>
where
    T: Out + 'own,
{
    fn slice(&self) -> &[T] {
        &self.0
    }

    fn mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }
}
impl<'own, T> OutCollection<'own, T> for OutCollectionSlice<'own, T>
where
    T: Out + 'own,
{
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool = true;
    const HAS_SORT_UNSTABLE: bool = true;
    const HAS_SORT: bool = false;

    fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity), PhantomData)
    }
    fn clear(&mut self) {
        self.0.clear();
    }

    fn len(&self) -> usize {
        self.slice().len()
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.slice().iter()
    }
    fn is_sorted(&self) -> bool {
        let iter = self.slice().iter();
        core::hint::black_box(iter).is_sorted()
    }
    fn sort(&mut self) {
        self.mut_slice().sort();
    }
    fn sort_unstable(&mut self) {
        unreachable!();
    }
    fn binary_search(&self, x: &T) -> bool {
        self.slice().binary_search(x).is_ok()
    }
}

pub struct OutCollectionSliceIndicator();
impl OutCollectionIndicator for OutCollectionSliceIndicator {
    type OutCollectionImpl<'own, T> = OutCollectionSlice<'own, T> where T: Out + 'own;
}
// End of: mut slice-based collection

type OutCollRetrieverPerItem<'own, OutCollectionIndicatorImpl, T> =
    <OutCollectionIndicatorImpl as OutCollectionIndicator>::OutCollectionImpl<'own, T>;

pub type OutRetriever<'own, OutIndicatorIndicatorImpl, Sub> =
    <<OutIndicatorIndicatorImpl as OutIndicatorIndicator>::OutIndicatorImpl<
        'own,
        Sub,
    > as OutIndicator<'own, Sub>>::OutLifetimedImpl;

pub type OutCollRetriever<'own, OutCollectionIndicatorImpl, OutIndicatorIndicatorImpl, Sub> =
    OutCollRetrieverPerItem<
        'own,
        OutCollectionIndicatorImpl,
        OutRetriever<'own, OutIndicatorIndicatorImpl, Sub>,
    >;
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

pub struct OutIndicatorStr<Sub>(PhantomData<Sub>);
/// `&str` is special, and so is this. Hence `Sub` is NOT used.
impl<'own, Sub> OutIndicator<'own, Sub> for OutIndicatorStr<Sub>
where
    Sub: OutLifetimed<'own>,
{
    type OutLifetimedImpl = &'own str;
}
pub struct OutIndicatorStrIndicator();
/// `&str` is special, and so is this. Hence `Sub` is NOT used.
impl OutIndicatorIndicator for OutIndicatorStrIndicator {
    type OutIndicatorImpl<'own, T> = OutIndicatorStr<T> where T: OutLifetimed<'own>;
}
