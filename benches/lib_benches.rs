// We need `path` attributes here, because this file itself is loaded this way, too.
#[path = "lib_benches/iaiish.rs"]
pub mod iaiish;

#[path = "lib_benches/criterionish.rs"]
pub mod criterionish;

#[path = "lib_benches/outish.rs"]
pub mod outish;

#[path = "lib_benches/shared.rs"]
pub mod shared;
