use core::marker::PhantomData;

use cgp_component::{IsProviderFor, WithProvider};
use cgp_macro::cgp_provider;

use crate::TypeComponent;
use crate::traits::ProvideType;

/**
    The `UseType` pattern is used to implement a CGP abstract type with the
    specified `Type`.

    When a CGP type component is defined using the `#[cgp_type]` macro, a
    provider implementation of `UseType` is automatically generated.
    With `UseType`, users can instantiate an abstract type with the specified
    `Type` without having to manually implement the given provider trait.

    ## Example

    Given the following type component definition:

    ```rust,ignore
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }
    ```

    The following `UseType` implementation would be generated:

    ```rust,ignore
    impl<Context, Type> NameTypeProvider<Context> for UseType<Type> {
        type Name = Type;
    }
    ```
*/
pub struct UseType<Type>(pub PhantomData<Type>);

pub type WithType<Type> = WithProvider<UseType<Type>>;

#[cgp_provider(TypeComponent)]
impl<Context, Tag, Type> ProvideType<Context, Tag> for UseType<Type> {
    type Type = Type;
}
