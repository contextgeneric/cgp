use cgp_error::{ErrorWrapper, HasErrorType};

pub struct DiscardDetail;

impl<Context, Detail> ErrorWrapper<Context, Detail> for DiscardDetail
where
    Context: HasErrorType,
{
    fn wrap_error(error: Context::Error, _detail: Detail) -> Context::Error {
        error
    }
}
