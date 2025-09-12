use core::marker::PhantomData;
use core::ops::Deref;

use cgp_component::UseContext;

/**
    The `HasField` trait is used to implement getter methods for a type that
    derives the trait.

    When a struct uses `#[derive(HasField)]`, the macro would generate a `HasField`
    implementation for each field in the struct, with the field name becoming a
    type-level string to be used in the generic `Tag` parameter.

    ## Example

    Given the following struct:

    ```rust,ignore
    #[derive(HasField)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }
    ```

    The macro would generate the following implementation:

    ```rust,ignore
    impl HasField<Symbol!("name")> for Person {
        type Value = String;

        fn get_field(&self, _tag: PhantomData<Symbol!("name")>) -> &Self::Value {
            &self.name
        }
    }

    impl HasField<Symbol!("age")> for Person {
        type Value = u8;

        fn get_field(&self, _tag: PhantomData<Symbol!("age")>) -> &Self::Value {
            &self.age
        }
    }
    ```
*/
#[diagnostic::on_unimplemented(
    message = "HasField is not implemented for {Self} with the field: {Tag}",
    note = "You need to add #[derive(HasField)] to {Self} with the given field present in the struct"
)]
pub trait HasField<Tag> {
    type Value;

    fn get_field(&self, _tag: PhantomData<Tag>) -> &Self::Value;
}

pub trait FieldGetter<Context, Tag> {
    type Value;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value;
}

#[diagnostic::do_not_recommend]
impl<Context, Tag, Target, Value> HasField<Tag> for Context
where
    Context: DerefMap<Target = Target>,
    Target: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value {
        self.map_deref(|context| context.get_field(tag))
    }
}

impl<Context, Tag, Field> FieldGetter<Context, Tag> for UseContext
where
    Context: HasField<Tag, Value = Field>,
{
    type Value = Field;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value {
        context.get_field(PhantomData)
    }
}

/**
   A helper trait to help organize the lifetime inference in Rust.
   Without this, `Self::Target` would need to be `'static`, as Rust couldn't
   infer the correct lifetime when calling `context.deref().get_field()`.
*/
trait DerefMap: Deref {
    fn map_deref<T>(&self, mapper: impl for<'a> FnOnce(&'a Self::Target) -> &'a T) -> &T;
}

impl<Context> DerefMap for Context
where
    Context: Deref,
{
    fn map_deref<T>(&self, mapper: impl for<'a> FnOnce(&'a Self::Target) -> &'a T) -> &T {
        mapper(self.deref())
    }
}
