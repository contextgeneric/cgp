//! An array key and a nested `new` table combine in one entry.
//!
//! Several keys share one nested `UseDelegate<new InnerComponents { … }>` value.
//! This is a compile-time check that the array-key plus nested-`new` forms parse
//! and expand together.
//!
//! See docs/reference/macros/delegate_components.md.

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

pub struct FooKey;
pub struct BarKey;
pub struct BazKey;

delegate_components! {
    new MyComponents {
        [
            FooKey,
            BarKey,
            BazKey,
        ]:
            UseDelegate<new InnerComponents {
                u32: String,
                u64: bool,
            }>,
    }
}
