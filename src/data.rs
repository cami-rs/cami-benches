use crate::outish::Out;
use crate::rnd::{data_own, Random};
use alloc::collections::BTreeSet;
use core::ops::RangeBounds;

extern crate alloc;

/*const MAX_CACHE_SIZE: usize = 10_000_000;

pub fn purge_cache() {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(core::hint::black_box(1));
    }
    core::hint::black_box(vec);
}*/

pub trait Data: Sized {
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8;
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize;
    fn string(&mut self) -> String;
    /// Param `range` is a range of length of the result [String], however, NOT in bytes, but in
    /// CHARACTERS.
    fn string_for_range(&mut self, range: impl RangeBounds<usize>) -> String;
    //fn strings
}

/// Stores (static, leaked) "own" & "out" data, where "out" potentially borrows from "own". When an
/// instance (of this struct) is stored in a (`static`) [once_cell::sync::OnceCell], these two
/// references add one level of indirection compare to having two separate (`static`)
/// [once_cell::sync::OnceCell] instances (which would mean more synchronization). But we want
/// simplicity.
pub struct OwnAndOut<OwnType: 'static, OutType: Out + 'static> {
    pub own: &'static [OwnType],
    /// Unsorted.
    pub out: &'static [OutType],
}

impl<OwnType: 'static, OutType: Out + 'static> OwnAndOut<OwnType, OutType> {
    pub fn new<Rnd: Random>(
        generate_own_item: impl Fn(&mut Rnd) -> OwnType,
        generate_out_item: impl Fn(&'static OwnType) -> OutType,
        allows_multiple_equal_items: bool,
    ) -> Self {
        Self::new_for(
            &mut Rnd::with_seed(),
            generate_own_item,
            generate_out_item,
            allows_multiple_equal_items,
        )
    }

    pub fn new_for<DataImpl: Data>(
        data: &mut DataImpl,
        generate_own_item: impl Fn(&mut DataImpl) -> OwnType,
        generate_out_item: impl Fn(&'static OwnType) -> OutType,
        allows_multiple_equal_items: bool,
    ) -> Self {
        let own = data_own(data, generate_own_item).leak();

        let mut out: Vec<OutType> = Vec::<OutType>::with_capacity(own.len());
        out.extend(own.iter().map(generate_out_item));

        if !allows_multiple_equal_items {
            let len_including_duplicates = out.len();
            // Remove duplicates. Yes, the result may have fewer items than planned/configured.
            let mut set = BTreeSet::<OutType>::new();
            set.extend(out.drain(..));
            out.extend(set.into_iter());
        }
        let out = out.leak();
        Self { own, out }
    }
}
