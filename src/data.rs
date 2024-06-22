use crate::outish::Out;
use crate::rnd::{self, Random};
use alloc::collections::BTreeSet;
use core::ops::Range;
use core::str::FromStr;

extern crate alloc;

/*const MAX_CACHE_SIZE: usize = 10_000_000;

pub fn purge_cache() {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(core::hint::black_box(1));
    }
    core::hint::black_box(vec);
}*/

pub trait OptAsData {
    fn as_data(&mut self) -> &mut dyn Data {
        panic!("");
    }
}

pub trait DataItems: OptAsData {
    fn num_items(&mut self) -> usize {
        self.as_data().usize(min_items()..max_items())
    }
}

impl<T: Data> OptAsData for T {
    fn as_data(&mut self) -> &mut dyn Data {
        self
    }
}

/// All default implementations of functions [panic].
pub trait Data: DataItems {
    fn u8(&mut self, range: Range<u8>) -> u8 {
        unimplemented!()
    }
    fn char(&mut self) -> char {
        unimplemented!()
    }
    fn usize(&mut self, range: Range<usize>) -> usize {
        unimplemented!()
    }
    fn string(&mut self) -> String {
        self.string_for_len_range(min_item_len()..max_item_len())
    }
    /// Param `range` is a range of length of the result [String], however, NOT in bytes, but in
    /// CHARACTERS.
    fn string_for_len_range(&mut self, range: Range<usize>) -> String {
        let num_chars = self.usize(range);
        let mut result = String::with_capacity(4 * num_chars);
        for _ in 0..num_chars {
            result.push(self.char());
        }
        result.shrink_to_fit();
        result
    }
}
//--------

/// Min number of test items (before removing duplicates).
const MIN_ITEMS: usize = 1000;

/// Max. number of test items.
const MAX_ITEMS: usize = 500000;

/// Min length of an item (where an item itself is a [Vec], [String]...). For example, for String,
/// this is the minimum number of `char`s - so the actual UTF-8 minimum length may be up to four
/// times higher.
const MIN_ITEM_LEN: usize = 1;

/// Max length of an item (where an item itself is a [Vec], [String]...). For example, for String,
/// this is the maximum number of `char`s - so the actual UTF-8 maximum length may be up to four
/// times higher.
const MAX_ITEM_LEN: usize = 1_000;

/// Parse a decimal value from an environment variable. If not present, use `otherwise`.
fn from_env_or(env_var_name: &str, otherwise: usize) -> usize {
    let env = std::env::var(env_var_name);
    if let Ok(st) = env {
        let env = usize::from_str(&st);
        if let Ok(env) = env {
            return env;
        }
        panic!("Environment variable {env_var_name} should be a 64-bit unsigned integer in decimal representation, but received {st}.");
    }
    otherwise
}

const MIN_ITEMS_ENV: &str = "MIN_ITEMS";
const MAX_ITEMS_ENV: &str = "MAX_ITEMS";
const MIN_ITEM_LEN_ENV: &str = "MIN_ITEM_LEN";
const MAX_ITEM_LEN_ENV: &str = "MAX_ITEM_LEN";
pub fn min_items() -> usize {
    from_env_or(MIN_ITEMS_ENV, MIN_ITEMS)
}
pub fn max_items() -> usize {
    from_env_or(MAX_ITEMS_ENV, MAX_ITEMS)
}
fn min_item_len() -> usize {
    from_env_or(MIN_ITEM_LEN_ENV, MIN_ITEM_LEN)
}
fn max_item_len() -> usize {
    from_env_or(MAX_ITEM_LEN_ENV, MAX_ITEM_LEN)
}
//--------

pub fn data_own<OwnType, DataImpl: Data>(
    data: &mut DataImpl,
    generate_own_item: impl Fn(&mut DataImpl) -> OwnType,
) -> Vec<OwnType> {
    let num_items = data.num_items();
    let mut own_items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        let item = generate_own_item(data);
        own_items.push(item);
    }
    own_items
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
    pub fn new_for_rnd<Rnd: Random>(
        generate_own_item: impl Fn(&mut Rnd) -> OwnType,
        generate_out_item: impl Fn(&'static OwnType) -> OutType,
        allows_multiple_equal_items: bool,
    ) -> Self {
        Self::new_for_data(
            &mut Rnd::with_seed(),
            generate_own_item,
            generate_out_item,
            allows_multiple_equal_items,
        )
    }

    pub fn new_for_data<DataImpl: Data>(
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
