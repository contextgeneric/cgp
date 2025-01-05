use core::marker::PhantomData;

use cgp_core::component::WithProvider;
use cgp_core::field::FieldGetter;
use cgp_core::prelude::*;

use crate::HasRuntimeType;

#[cgp_component {
    provider: RuntimeGetter,
}]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}

impl<Context, Provider, Runtime> RuntimeGetter<Context> for WithProvider<Provider>
where
    Context: HasRuntimeType<Runtime = Runtime>,
    Provider: FieldGetter<Context, RuntimeGetterComponent, Value = Runtime>,
{
    fn runtime(context: &Context) -> &Runtime {
        Provider::get_field(context, PhantomData)
    }
}
