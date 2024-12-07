#[cfg(feature = "send")]
pub use core::marker::Send as MaybeSend;

#[cfg(not(feature = "send"))]
pub trait MaybeSend {}

#[cfg(not(feature = "send"))]
impl<T> MaybeSend for T {}
