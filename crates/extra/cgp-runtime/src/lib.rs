//! This crate provides CGP's runtime abstraction, which lets a context choose its async runtime.
//!
//! See [`HasRuntime`](https://contextgeneric.dev/docs/reference/components/has_runtime) and [`HasRuntimeType`](https://contextgeneric.dev/docs/reference/components/has_runtime_type).

mod traits;

pub use traits::{
    HasRuntime, HasRuntimeType, RuntimeGetter, RuntimeGetterComponent, RuntimeOf,
    RuntimeTypeProvider, RuntimeTypeProviderComponent,
};
