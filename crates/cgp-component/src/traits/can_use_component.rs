use crate::{HasComponents, IsProviderFor};

pub trait CanUseComponent<Component, Params = ()> {}

impl<Context, Component, Params> CanUseComponent<Component, Params> for Context
where
    Context: HasComponents,
    Context::Components: IsProviderFor<Component, Context, Params>,
{
}
