#[cfg(feature = "static")]
pub trait MaybeStatic: 'static {}

#[cfg(feature = "static")]
impl<T: 'static> MaybeStatic for T {}

#[cfg(not(feature = "static"))]
pub trait MaybeStatic {}

#[cfg(not(feature = "static"))]
impl<T> MaybeStatic for T {}
