use cgp_core::component::WithProvider;
use cgp_core::prelude::*;
use cgp_core::types::traits::ProvideType;

#[cgp_component {
    name: RuntimeTypeComponent,
    provider: ProvideRuntimeType,
}]
pub trait HasRuntimeType {
    type Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

impl<Context, Provider, Runtime> ProvideRuntimeType<Context> for WithProvider<Provider>
where
    Provider: ProvideType<Context, RuntimeTypeComponent, Type = Runtime>,
{
    type Runtime = Runtime;
}
