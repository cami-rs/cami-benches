#![feature(extend_one)]
#![feature(is_sorted)]
#![feature(trait_alias)]
#![feature(coroutines, coroutine_trait)]
#![feature(stmt_expr_attributes)]

pub mod col;
pub mod data;
pub mod outish;
pub mod rnd;

// Feature-based:

#[cfg(feature = "criterion")]
pub mod shared_criterion;

#[cfg(feature = "iai-callgrind")]
pub mod shared_iai;

#[cfg(test)]
pub mod testish;
