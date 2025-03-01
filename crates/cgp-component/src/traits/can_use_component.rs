use crate::{HasProvider, IsProviderFor};

pub trait CanUseComponent<Component, Params = ()> {}

impl<Context, Component, Params> CanUseComponent<Component, Params> for Context
where
    Context: HasProvider,
    Context::Provider: IsProviderFor<Component, Context, Params>,
{
}

pub trait CheckCanUseComponent<Component, Params = ()>: CanUseComponent<Component, Params> {}
