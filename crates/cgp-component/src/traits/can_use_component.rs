use crate::{HasCgpProvider, IsProviderFor};

/**
    This is a convenient type alias that is used in the same way as [`IsProviderFor`],
    but with the `Self` type being the `Context` type rather than the `Provider` type
    that implements the provider trait.

    The `CanUseComponent` trait is automatically implemented for any CGP `Context` type
    that implements the `HasCgpProvider` trait, and when `Contex::CgpProvider` implements
    `IsProviderFor<Component, Context, Params>`.

    This trait is used by `check_components!` to check whether a `Context` implements
    a given `Component` through its provider. When there are unsatisfied constraints,
    Rust would show the error messages from the `IsProviderFor` implementation.
*/
pub trait CanUseComponent<Component, Params: ?Sized = ()> {}

impl<Context, Component, Params> CanUseComponent<Component, Params> for Context
where
    Context: HasCgpProvider,
    Context::CgpProvider: IsProviderFor<Component, Context, Params>,
{
}
