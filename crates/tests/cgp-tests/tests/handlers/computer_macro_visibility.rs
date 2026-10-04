//! A `#[cgp_computer]` function declared in a module and used from outside it.
//!
//! The generated provider struct is public, so a `pub fn` computer defined in a
//! child module can be named and wired from its parent.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_computer.md.

use cgp::extra::handler::Computer;
use cgp::prelude::*;

mod math {
    use cgp::prelude::*;

    #[cgp_computer]
    pub fn double(value: u64) -> u64 {
        value * 2
    }
}

pub struct App;

#[test]
fn test_computer_across_modules() {
    assert_eq!(math::Double::compute(&App, PhantomData::<()>, 2), 4);
}
