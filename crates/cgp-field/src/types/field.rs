use core::fmt::Debug;
use core::marker::PhantomData;

/**
   The `Field` type, a.k.a. `ω`, is used to represent a _named_ field entry
   within a product type or a sum type.

   `Field` is parameterized by a phantom `Tag` type, which is used to represent
   the field name as type. Typically, this would either be a type-level string
   such as `Symbol!("name")`, or a type-level index such as `Index<0>`.
   Aside from that, `Field` is essentially a wrapper around `Value`.

   `Field` is mainly used within the derived [`HasFields`](crate::traits::HasFields)
   implementations, to include the field name in the generic product or sum
   representation of the given struct or enum.

   `Field` is also shown as `ω` to improve the readability of compiler error
   messages. It is mainly useful when the type from `HasFields::Fields` is shown,
   which would contain a lot of `Field`s and tend to take up a lot of screen space
   to read.

   ## Example

   Given the following struct definition:

   ```rust,ignore
   #[derive(HasFields)]
   pub struct MyContext {
       pub name: String,
       pub age: u8,
   }
   ```

   The following `HasFields` implementation would be generated:

   ```rust,ignore
   impl HasFields for MyContext {
       type Fields = Product![Field<Symbol!("name"), String>, Field<Symbol!("age"), u8>];
   }
   ```

   which would be shown with the shortened representation as:

   ```rust,ignore
   impl HasFields for MyContext {
       type Fields =
           π<ω<Symbol!("name"), String>,
               π<ω<Symbol!("age"), u8>,
                   ε>>;
   }
   ```
*/
pub struct ω<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}

pub use ω as Field;

impl<Tag, Value> From<Value> for Field<Tag, Value> {
    fn from(value: Value) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<Tag, Value> Debug for Field<Tag, Value>
where
    Value: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<Tag, Value> PartialEq for Field<Tag, Value>
where
    Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<Tag, Value> Eq for Field<Tag, Value>
where
    Value: Eq,
{
    fn assert_receiver_is_total_eq(&self) {
        self.value.assert_receiver_is_total_eq()
    }
}
