use cgp_fork::prelude::*;

use crate::HasRuntimeType;

#[cgp_getter]
#[use_type(HasRuntimeType.Runtime)]
pub trait HasRuntime {
    fn runtime(&self) -> &Runtime;
}
