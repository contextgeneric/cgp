//! This crate provides CGP's type-level primitives: symbols, lists, sums, and paths.
//!
//! See [`Symbol!`](https://contextgeneric.dev/docs/reference/macros/symbol), [`Cons`](https://contextgeneric.dev/docs/reference/types/cons), [`Nil`](https://contextgeneric.dev/docs/reference/types/nil), [`Chars`](https://contextgeneric.dev/docs/reference/types/chars), [`PathCons`](https://contextgeneric.dev/docs/reference/types/path_cons), [`StaticFormat`](https://contextgeneric.dev/docs/reference/traits/formatting/static_format), and [`ConcatPath`](https://contextgeneric.dev/docs/reference/traits/formatting/concat_path).
#![no_std]

pub mod macro_prelude;
pub mod traits;
pub mod types;
