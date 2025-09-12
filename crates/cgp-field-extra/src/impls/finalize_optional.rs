use alloc::string::{String, ToString};
use core::fmt::Display;
use core::marker::PhantomData;

use cgp_field::impls::{IsNothing, IsOptional};
use cgp_field::traits::{BuildField, FinalizeBuild, HasFields, PartialData, UpdateField};
use cgp_field::types::{Cons, Field, Nil};

pub trait FinalizeOptional: PartialData {
    fn finalize_optional(self) -> Result<Self::Target, String>;
}

impl<ContextA, ContextB, Target> FinalizeOptional for ContextA
where
    ContextA: PartialData<Target = Target>,
    Target: HasFields,
    Target::Fields: FinalizeOptionalImpl<ContextA, Output = ContextB>,
    ContextB: FinalizeBuild<Target = Target>,
{
    fn finalize_optional(self) -> Result<Self::Target, String> {
        let context = Target::Fields::finalize_optional(self)?;
        Ok(context.finalize_build())
    }
}

trait FinalizeOptionalImpl<Context> {
    type Output;

    fn finalize_optional(context: Context) -> Result<Self::Output, String>;
}

impl<Tag, Value, Rest, ContextA, ContextB, ContextC, ContextD> FinalizeOptionalImpl<ContextA>
    for Cons<Field<Tag, Value>, Rest>
where
    Rest: FinalizeOptionalImpl<ContextA, Output = ContextB>,
    ContextB: UpdateField<Tag, IsNothing, Mapper = IsOptional, Value = Value, Output = ContextC>,
    ContextC: BuildField<Tag, Value = Value, Output = ContextD>,
    Tag: Default + Display,
{
    type Output = ContextD;

    fn finalize_optional(context: ContextA) -> Result<Self::Output, String> {
        let context = Rest::finalize_optional(context)?;
        let (m_value, context) = context.update_field(PhantomData, ());

        let value = m_value.ok_or_else(|| Tag::default().to_string())?;
        let context = context.build_field(PhantomData, value);

        Ok(context)
    }
}

impl<Context> FinalizeOptionalImpl<Context> for Nil {
    type Output = Context;

    fn finalize_optional(context: Context) -> Result<Self::Output, String> {
        Ok(context)
    }
}
