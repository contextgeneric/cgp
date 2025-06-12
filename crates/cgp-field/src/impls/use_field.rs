use core::marker::PhantomData;

use cgp_component::{IsProviderFor, WithProvider};
use cgp_macro::cgp_provider;
use cgp_type::{ProvideType, TypeComponent};

use crate::traits::{FieldGetter, HasField, HasFieldMut, MutFieldGetter};

/**
    The `UseField` pattern is used to implement a CGP getter trait by reading
    the `Tag` field from `Context` via the [`HasField`] trait.

    When a CGP getter component contains only one getter method and is defined
    using the `#[cgp_getter]` macro, a `UseField` implementation is automatically
    generated.

    Typically, the `Tag` type would be a type-level string defined through
    the `symbol!` macro, such as `symbol!("name")`. It may also be a type-level
    integer that is wrapped in the `Index` type, such as `Index<0>`.
    The `HasField` implementation for these tag types would be automatically
    implemented following the field names in the struct when `#[derive(HasField)]`
    is used.

    However, users are free to use any type as the `Tag` type, as there is no
    additional constraints on what the type should be. The only consequence is
    that such manual tags would not be automatically implemented by
    `#[derive(HasField)]`, and so users would have to manually implement the
    `HasField` instances, or generate them from a different macro.

    `UseField` allows users to easily implement a getter trait for a context
    by only having to derive the [`HasField`] trait, and specifying the field
    name through `UseField`. This reduces the amount of boilerplate code required
    to manually implement the getter trait for each concrete context.

    ## Example

    Given the following getter component definition:

    ```rust,ignore
    #[cgp_getter(NameGetter)]
    pub trait HasName {
        fn name(&self) -> &str;
    }
    ```

    The following `UseField` implementation would be generated:

    ```rust,ignore
    impl<Context, Tag> NameGetter<Context> for UseField<Tag>
    where
        Context: HasField<Tag, Value = String>,
    {
        fn name(context: &Context) -> &str {
            &context.get_field(PhantomData).as_str()
        }
    }
    ```
*/
pub struct UseField<Tag>(pub PhantomData<Tag>);

pub type WithField<Tag> = WithProvider<UseField<Tag>>;

#[cgp_provider(TypeComponent)]
impl<Context, TypeTag, FieldTag, Field> ProvideType<Context, TypeTag> for UseField<FieldTag>
where
    Context: HasField<FieldTag, Value = Field>,
{
    type Type = Field;
}

impl<Context, OutTag, Tag, Value> FieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(context: &Context, _tag: PhantomData<OutTag>) -> &Value {
        context.get_field(PhantomData)
    }
}

impl<Context, OutTag, Tag, Value> MutFieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasFieldMut<Tag, Value = Value>,
{
    fn get_field_mut(context: &mut Context, _tag: PhantomData<OutTag>) -> &mut Value {
        context.get_field_mut(PhantomData)
    }
}
