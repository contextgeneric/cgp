//! `#[cgp_computer]` and `#[cgp_producer]` in a module that does not import
//! `cgp_fork::prelude::*`.
//!
//! The expansion names every CGP item through its fully qualified
//! `::cgp_fork::macro_prelude` path, so the macros work when invoked by path alone,
//! and an item of the same name as a CGP trait in the caller's module, here a
//! local `Computer` struct, does not capture it.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_computer.md.

use core::marker::PhantomData;

/// A local item named like the `Computer` trait the expansion implements.
pub struct Computer;

#[cgp_fork::prelude::cgp_computer]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cgp_fork::prelude::cgp_computer]
async fn add_async(a: u64, b: u64) -> u64 {
    a + b
}

#[cgp_fork::prelude::cgp_computer]
fn checked_add(a: u64, b: u64) -> Result<u64, String> {
    a.checked_add(b).ok_or_else(|| "overflow".to_owned())
}

#[cgp_fork::prelude::cgp_producer]
fn magic_number() -> u64 {
    42
}

pub struct App;

#[test]
fn test_macros_without_prelude() {
    use cgp_fork::extra::handler::{AsyncComputer, Computer, Producer};
    use futures::executor::block_on;

    assert_eq!(Add::compute(&App, PhantomData::<()>, (1, 2)), 3);
    assert_eq!(CheckedAdd::compute(&App, PhantomData::<()>, (1, 2)), Ok(3));
    assert_eq!(MagicNumber::produce(&App, PhantomData::<()>), 42);

    assert_eq!(
        block_on(AddAsync::compute_async(&App, PhantomData::<()>, (1, 2))),
        3
    );
}
