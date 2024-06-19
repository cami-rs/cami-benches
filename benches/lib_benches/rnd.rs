use core::num;
//use crate::lib_benches::
use core::ops::RangeBounds;
use core::str::FromStr;
#[cfg(feature = "fastrand")]
use fastrand::Rng;

/// If calling [data_out] with [[OutCollection] that has[OutCollection::ALLOWS_MULTIPLE_EQUAL_ITEMS]
/// set to `true`, then [MIN_ITEMS_AFTER_REMOVING_DUPLICATES] is the minimum number of items
/// required for benchmarking to continue. Otherwise we get a [panic].
pub const MIN_ITEMS_AFTER_REMOVING_DUPLICATES: usize = 4;

/// Min number of test items.
pub const MIN_ITEMS: usize = 500000;
/// Max. number of test items.
pub const MAX_ITEMS: usize = 5000000;

/// On heap. For example, for String, this is the minimum number of `char`s - so the actual UTF-8
/// size may be up to four times higher.
pub const MIN_ITEM_LEN: usize = 1;

/// On heap. For example, for String, this is the maximum number of `char`s - so the actual UTF-8
/// size may be up to four times higher.
pub const MAX_ITEM_LEN: usize = 1_000;

/// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;
//------

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
    fn string(&mut self) -> String;
    /// Param `range` is a range of length of the result [String], however, NOT in bytes, but in
    /// CHARACTERS.
    fn string_for_range(&mut self, range: impl RangeBounds<usize>) -> String;
}

impl Random for Rng {
    fn with_seed_dec(seed: &str) -> Self {
        Rng::with_seed(u64::from_str(seed).expect("Environment variable RND_SEED_DEC should be a 64-bit unsigned integer in decimal representation."))
    }
    fn with_seed_hex(seed: &str) -> Self {
        Rng::with_seed(u64::from_str_radix(seed, 16).expect("Environment variable RND_SEED_HEX should be a 64-bit unsigned integer in hexadecimal representation."))
    }

    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        Rng::usize(self, range)
    }
    fn string(&mut self) -> String {
        self.string_for_range(MIN_ITEMS..MAX_ITEMS)
    }
    fn string_for_range(&mut self, range: impl RangeBounds<usize>) -> String {
        let num_chars = self.usize(range);
        let mut result = String::with_capacity(4 * num_chars);
        for _ in 0..num_chars {
            result.push(Rng::alphanumeric(self));
        }
        result.shrink_to_fit();
        result
    }
}

pub const RND_SEED_DEC: &'static str = "RND_SEED_DEC";
pub const RND_SEED_HEX: &'static str = "RND_SEED_HEX";

pub fn data_own_for_rnd<OwnType, Rnd: Random>(
    generate_own_item: impl Fn(&mut Rnd) -> OwnType,
) -> Vec<OwnType> {
    let seed_dec = std::env::var(RND_SEED_DEC);
    let seed_hex = std::env::var(RND_SEED_HEX);
    if seed_dec.is_ok() && seed_hex.is_ok() {
        panic!("You've provided both environment variables {RND_SEED_DEC}: {} and {RND_SEED_HEX}: {}, but this requires exactly one.", seed_dec.unwrap(), seed_hex.unwrap());
    }
    let mut rnd = if let Ok(dec) = seed_dec {
        Rnd::with_seed_dec(&dec)
    } else if let Ok(hex) = seed_hex {
        Rnd::with_seed_hex(&hex)
    } else {
        panic!("Requiring exactly one of two environment variables RND_SEED_DEC, RND_SEED_HEX, but received none.");
    };

    let num_items = rnd.usize(MIN_ITEMS..MAX_ITEMS);
    let mut own_items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        let item = generate_own_item(&mut rnd);
        own_items.push(item);
    }
    own_items
}

#[cfg(feature = "fastrand")]
pub type RndChoice = Rng;

#[cfg(not(feature = "fastrand"))]
compile_error!("Currently we require 'fastrand' feature.");

pub fn data_own<OwnType>(generate_own_item: impl Fn(&mut RndChoice) -> OwnType) -> Vec<OwnType> {
    data_own_for_rnd::<OwnType, RndChoice>(generate_own_item)
}
