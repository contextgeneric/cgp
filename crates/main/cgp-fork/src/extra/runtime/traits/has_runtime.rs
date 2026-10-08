use crate::core::prelude::*;
use crate::extra::runtime::HasRuntimeType;

#[cgp_getter]
#[use_type(HasRuntimeType.Runtime)]
pub trait HasRuntime {
    fn runtime(&self) -> &Runtime;
}
