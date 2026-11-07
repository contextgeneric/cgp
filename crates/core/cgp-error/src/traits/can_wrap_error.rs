use cgp::component::{DefaultNamespace, UseDelegate};
use cgp_macro::cgp_component;

use crate::traits::HasErrorType;

/// The component that folds a piece of detail, such as a message or a path, into an error the
/// context already holds, dispatching per detail type.
///
/// See [`CanWrapError`](https://contextgeneric.dev/docs/reference/components/can_wrap_error).
#[cgp_component(ErrorWrapper)]
#[prefix(@cgp.core.error in DefaultNamespace)]
#[derive_delegate(UseDelegate<Detail>)]
#[use_type(HasErrorType.Error)]
pub trait CanWrapError<Detail> {
    /// `#[track_caller]` here applies to every provider impl and to the generated forwarding
    /// impls, so an error library that records `Location::caller()` sees the caller's line.
    #[track_caller]
    fn wrap_error(error: Error, detail: Detail) -> Error;
}
