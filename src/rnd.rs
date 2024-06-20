use crate::data::Data;
use core::num;
use core::ops::RangeBounds;
use core::str::FromStr;
#[cfg(feature = "fastrand")]
use fastrand::Rng;

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

pub const MIN_ITEMS_ENV: &str = "MIN_ITEMS";
pub const MAX_ITEMS_ENV: &str = "MAX_ITEMS";
pub const MIN_ITEM_LEN_ENV: &str = "MIN_ITEM_LEN";
pub const MAX_ITEM_LEN_ENV: &str = "MAX_ITEM_LEN";
fn min_items() -> usize {
    from_env_or(MIN_ITEMS_ENV, MIN_ITEMS)
}
fn max_items() -> usize {
    from_env_or(MAX_ITEMS_ENV, MAX_ITEMS)
}
fn min_item_len() -> usize {
    from_env_or(MIN_ITEM_LEN_ENV, MIN_ITEM_LEN)
}
fn max_item_len() -> usize {
    from_env_or(MAX_ITEM_LEN_ENV, MAX_ITEM_LEN)
}

//------

pub const RND_SEED_DEC_ENV: &str = "RND_SEED_DEC";
pub const RND_SEED_HEX_ENV: &str = "RND_SEED_HEX";

/// We create one instance per set of compared benchmarks. We don't re-use the same instance for all
/// benchmarks, because we'd need mutable access to such instance, and that's tricky with
/// `iai-callgrind`'s, `Criterion`'s or other harness's macros. That would prevent benchmarking in
/// parallel.
///
/// Therefore we require the user to provide a seed.
pub trait Random: Data + Sized {
    /// Initiate with a seed. The seed is parsed from `seed`, which is in decimal representation.
    /// (It comes from environment variable `RND_SEED_DEC`).
    ///
    /// The actual format of `seed` depends on the implementation (it may be one `u64`, multiple
    /// `u64`'s separated by whitespace, or other.)
    fn with_seed_dec(seed: &str) -> Self;
    /// Initiate with a seed. The seed is parsed from `seed`, which is in hexadecimal
    /// representation. (It comes from environment variable `RND_SEED_HEX`).
    ///
    /// The actual format of `seed` depends on the implementation (it may be one `u64`, multiple
    /// `u64`'s separated by whitespace, or other.)
    fn with_seed_hex(seed: &str) -> Self;

    /// Initiate with a seed, by default from an environment variable `RND_SEED_DEC` or
    /// `RND_SEED_HEX` - see [Random::with_seed_dec] and [Random::with_seed_hex]. Override this only
    /// for tests or special.
    fn with_seed() -> Self {
        let seed_dec = std::env::var(RND_SEED_DEC_ENV);
        let seed_hex = std::env::var(RND_SEED_HEX_ENV);
        if seed_dec.is_ok() && seed_hex.is_ok() {
            panic!("You've provided both environment variables {RND_SEED_DEC_ENV}: {} and {RND_SEED_HEX_ENV}: {}, but this requires exactly one.", seed_dec.unwrap(), seed_hex.unwrap());
        }
        if let Ok(dec) = seed_dec {
            Self::with_seed_dec(&dec)
        } else if let Ok(hex) = seed_hex {
            Self::with_seed_hex(&hex)
        } else {
            panic!("Requiring exactly one of two environment variables RND_SEED_DEC, RND_SEED_HEX, but received none.");
        }
    }
}

#[cfg(feature = "fastrand")]
impl Data for Rng {
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        Rng::usize(self, range)
    }
    fn string(&mut self) -> String {
        self.string_for_range(min_item_len()..max_item_len())
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

#[cfg(feature = "fastrand")]
impl Random for Rng {
    fn with_seed_dec(seed: &str) -> Self {
        Rng::with_seed(u64::from_str(seed).expect("Environment variable RND_SEED_DEC should be a 64-bit unsigned integer in decimal representation."))
    }
    fn with_seed_hex(seed: &str) -> Self {
        Rng::with_seed(u64::from_str_radix(seed, 16).expect("Environment variable RND_SEED_HEX should be a 64-bit unsigned integer in hexadecimal representation."))
    }
}

pub fn data_own<OwnType, DataImpl: Data>(
    rnd: &mut DataImpl,
    generate_own_item: impl Fn(&mut DataImpl) -> OwnType,
) -> Vec<OwnType> {
    let num_items = rnd.usize(min_items()..max_items());
    let mut own_items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        let item = generate_own_item(rnd);
        own_items.push(item);
    }
    own_items
}
