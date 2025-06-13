use crate::traits::send::MaybeSend;
use crate::traits::sync::MaybeSync;

/**
   The `Async` trait is a convenient trait alias for `Send + Sync`.

   It is mainly used as the default constraint for abstract types
   and generic types in CGP, together with the `#[async_trait]` macro
   to make the returned `Future` from async functions to implement `Send`.

   Technically, `Async` is actually a trait alias for [`MaybeSend`]
   and [`MaybeSync`], which are aliases to `Send` and `Sync` when the
   respective `send` and `sync` feature flags are enabled are enabled
   in the `cgp-async` crate.

   This provides a semi-reliable way for CGP applications to turn off
   the `send` and `sync` features, if the specific application does not
   require them. However, the application may need to ensure that the
   feature is also disabled in all its dependencies for it to work.
*/
pub trait Async: MaybeSend + MaybeSync {}

impl<A> Async for A where A: MaybeSend + MaybeSync {}
