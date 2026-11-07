use crate::{DelegateComponent, IsProviderFor};

/// The check a check_components! entry asserts: the context wires the component, and the provider
/// it chose has every dependency met for that context.
///
/// See
/// [`CanUseComponent`](https://contextgeneric.dev/docs/reference/traits/wiring/can_use_component) .
pub trait CanUseComponent<Component, Params: ?Sized = ()> {}

impl<Context, Component, Params: ?Sized> CanUseComponent<Component, Params> for Context
where
    Context: DelegateComponent<Component>,
    Context::Delegate: IsProviderFor<Component, Context, Params>,
{
}
