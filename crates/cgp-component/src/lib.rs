#![no_std]

/*!
   This crate defines the core CGP types and traits used to enable the core
   CGP component implementation.
*/

mod namespaces;
mod traits;
mod types;

pub use namespaces::{CgpCore, CgpNamespace, DefaultComponentsNamespace};
pub use traits::{AppendProduct, CanUseComponent, DelegateComponent, IsProviderFor};
pub use types::{
    ConcatPath, PathCons, PathNil, RedirectLookup, UseContext, UseDefault, UseDelegate, UseFields,
    WithContext, WithProvider,
};
