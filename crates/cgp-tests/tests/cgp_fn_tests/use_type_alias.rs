use std::ops::Mul;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_fn]
#[use_type(HasScalarType::{Scalar as S})]
pub fn rectangle_area(&self, #[implicit] width: S, #[implicit] height: S) -> S
where
    S: Mul<Output = S> + Clone,
{
    width * height
}
