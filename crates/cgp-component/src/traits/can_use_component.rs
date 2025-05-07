use crate::{HasCgpProvider, IsProviderFor};

pub trait CanUseComponent<Component, Params = ()> {}

impl<Context, Component, Params> CanUseComponent<Component, Params> for Context
where
    Context: HasCgpProvider,
    Context::CgpProvider: IsProviderFor<Component, Context, Params>,
{
}
