use crate::core::error::{ErrorWrapper, ErrorWrapperComponent, HasErrorType};
use crate::core::prelude::*;

#[cgp_new_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DiscardDetail
where
    Context: HasErrorType,
{
    fn wrap_error(error: Context::Error, _detail: Detail) -> Context::Error {
        error
    }
}
