//! `Struct!` and `Enum!` emit every CGP name they need through its fully
//! qualified path, so a module that imports nothing from `cgp_fork` can still use
//! them by path. The `without_prelude` module below has no `use` items at all.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/struct.md.

use cgp_fork::prelude::*;

use crate::shape_macros::assertions::assert_same_type;

mod without_prelude {
    pub type Point = cgp_fork::prelude::Struct! { x: f64, y: f64 };

    pub type Pair = cgp_fork::prelude::Struct!(u8, u16);

    pub type Choice = cgp_fork::prelude::Enum! { Left(u8), Right };
}

#[test]
fn the_shape_macros_need_no_imports() {
    assert_same_type::<
        without_prelude::Point,
        Product![Field<Symbol!("x"), f64>, Field<Symbol!("y"), f64>],
    >();

    assert_same_type::<without_prelude::Pair, Product![Field<Index<0>, u8>, Field<Index<1>, u16>]>(
    );

    assert_same_type::<
        without_prelude::Choice,
        Sum![Field<Symbol!("Left"), u8>, Field<Symbol!("Right"), Nil>],
    >();
}
