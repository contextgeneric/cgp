use cgp_core::component::WithProvider;
use cgp_core::field::FieldGetter;
use cgp_core::prelude::*;

use crate::HasRuntimeType;

#[cgp_getter {
    context: App,
    provider: RuntimeGetter,
}]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}
