#[cfg(feature = "provider-supertrait")]
pub const ENABLE_IS_PROVIDER_SUPERTRAIT: bool = true;

#[cfg(not(feature = "provider-supertrait"))]
pub const ENABLE_IS_PROVIDER_SUPERTRAIT: bool = false;
