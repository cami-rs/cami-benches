// We need `path` attributes here, because this file itself is loaded this way, too.

#[path = "lib_benches/col.rs"]
pub mod col;

#[path = "lib_benches/data.rs"]
pub mod data;

#[path = "lib_benches/rnd.rs"]
pub mod rnd;

#[path = "lib_benches/outish.rs"]
pub mod outish;

#[path = "lib_benches/shared.rs"]
pub mod shared;

// Feature-based:

#[cfg(feature = "criterion")]
#[path = "lib_benches/shared_criterion.rs"]
pub mod shared_criterion;

#[cfg(feature = "iai-callgrind")]
#[path = "lib_benches/shared_iai.rs"]
pub mod shared_iai;
