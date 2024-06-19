use crate::lib_benches::outish::Out;
use crate::lib_benches::rnd::{data_own, RndChoice, MIN_ITEMS_AFTER_REMOVING_DUPLICATES};
use alloc::collections::BTreeSet;

extern crate alloc;

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
