use cgp_component::{DelegateComponent, HasProvider, IsProviderFor, UseDelegate};
use cgp_component_macro::{cgp_component, cgp_provider};

use crate::traits::HasErrorType;

#[cgp_component {
    provider: ErrorWrapper,
}]
pub trait CanWrapError<Detail>: HasErrorType {
    fn wrap_error(error: Self::Error, detail: Detail) -> Self::Error;
}

#[cgp_provider(ErrorWrapperComponent)]
impl<Context, Detail, Components> ErrorWrapper<Context, Detail> for UseDelegate<Components>
where
    Context: HasErrorType,
    Components: DelegateComponent<Detail>,
    Components::Delegate: ErrorWrapper<Context, Detail>,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        Components::Delegate::wrap_error(error, detail)
    }
}
