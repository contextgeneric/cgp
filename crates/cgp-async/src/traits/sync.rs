#[cfg(feature = "sync")]
pub use core::marker::Sync as MaybeSync;

#[cfg(not(feature = "sync"))]
pub trait MaybeSync {}

#[cfg(not(feature = "sync"))]
impl<T> MaybeSync for T {}
