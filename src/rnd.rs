use crate::data::{self, Data, DataItems};
use core::num;
use core::ops::Range;
use core::str::FromStr;
#[cfg(feature = "fastrand")]
use fastrand::Rng;

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
    fn u8(&mut self, range: Range<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn char(&mut self) -> char {
        Rng::alphanumeric(self)
    }
    fn usize(&mut self, range: Range<usize>) -> usize {
        Rng::usize(self, range)
    }
}

impl DataItems for Rng {}
/*
    fn num_items(&mut self) -> usize {
        self.usize(data::min_items()..data::max_items())
    }
}*/

#[cfg(feature = "fastrand")]
impl Random for Rng {
    fn with_seed_dec(seed: &str) -> Self {
        Rng::with_seed(u64::from_str(seed).expect("Environment variable RND_SEED_DEC should be a 64-bit unsigned integer in decimal representation."))
    }
    fn with_seed_hex(seed: &str) -> Self {
        Rng::with_seed(u64::from_str_radix(seed, 16).expect("Environment variable RND_SEED_HEX should be a 64-bit unsigned integer in hexadecimal representation."))
    }
}
