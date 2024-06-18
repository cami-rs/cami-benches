// We need `path` attributes here, because this file itself is loaded this way, too.

#[cfg(feature = "criterion")]
#[path = "lib_benches/shared_criterion.rs"]
pub mod shared_criterion;

#[path = "lib_benches/outish.rs"]
pub mod outish;

#[path = "lib_benches/shared.rs"]
pub mod shared;

#[cfg(feature = "iai-callgrind")]
#[path = "lib_benches/shared_iai.rs"]
pub mod shared_iai;
