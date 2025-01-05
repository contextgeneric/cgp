use crate::traits::r#static::MaybeStatic;
use crate::traits::send::MaybeSend;
use crate::traits::sync::MaybeSync;

/**
   This is defined as a convenient constraint alias to
   `Send + Sync + 'static`.
*/
pub trait Async: MaybeSend + MaybeSync + MaybeStatic {}

impl<A> Async for A where A: MaybeSend + MaybeSync + MaybeStatic {}
