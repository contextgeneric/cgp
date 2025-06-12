/**
   This is the core trait used by the `delegate_component!` macro to define a mapping
   from `Name` to `Delegate` on the target `Self` type.

   ## `DelegateComponent` as a type-level key-value map

   Essentially, `DelegateComponent` turns the `Self` type into a type-level key-value map,
   with `Name` as the key and `Delegate` as the value. The implementation of
   `DelegateComponent` serves as "setting" an entry of `Self` at `Name` to `Delegate`.
   Then, the inclusion of `DelegateComponent` in a constraint serves as "getting" the
   `Delegate` value with the provided `Name` as the key.

   When `Name` type in `DelegateComponent` is a CGP component name type, then the `Self`
   type would also automatically implement the corresponding provider trait through the
   blanket implementation.

   However, it is also common to use `DelegateComponent` with regular types as the `Name`
   key, especially when it is used to define lookup tables for providers such as
   `UseDelegate`. In such cases, the `Self` type that implements `DelegateComponent` would
   not be used to implement any provider trait.

   ## Examples

   As an example, given the following `delegate_component!` macro invocation:

   ```rust,ignore
   delegate_component! {
       MyComponent {
           GreeterComponent: GreetHello,
       }
   }
   ```

   would generate the following impl:

   ```rust,ignore
   impl DelegateComponent<GreeterComponent> for MyComponent {
       type Delegate = GreetHello;
   }
   ```
*/
#[diagnostic::on_unimplemented(
    message = "{Self} does not contain any DelegateComponent entry for {Name}",
    note = "You might want to implement the provider trait for {Name} on {Self}"
)]
pub trait DelegateComponent<Name> {
    type Delegate;
}
