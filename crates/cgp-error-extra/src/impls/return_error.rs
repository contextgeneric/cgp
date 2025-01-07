use cgp_error::{ErrorRaiser, HasErrorType};

pub struct ReturnError;

impl<Context, E> ErrorRaiser<Context, E> for ReturnError
where
    Context: HasErrorType<Error = E>,
{
    fn raise_error(e: E) -> E {
        e
    }
}
