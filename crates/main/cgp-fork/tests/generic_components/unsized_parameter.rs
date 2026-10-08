//! A component whose type parameter is `?Sized`, wired through `delegate_components!`
//! at an unsized argument and then *called*, not only checked.
//!
//! `HasReference<'a, T: 'a + ?Sized>` used at `str` makes the `IsProviderFor` params
//! tuple `(Life<'a>, str)`, which is unsized. The provider blanket impl requires the
//! context itself to implement `IsProviderFor` through the table's forwarding impl, so
//! that impl must declare its `__Params__` parameter `?Sized` for the call to resolve;
//! `check_components!` alone would pass either way, since it asks the provider's own
//! `IsProviderFor` impl rather than the table's.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/delegate_components.md.

use cgp_fork::prelude::*;

#[cgp_component(ReferenceGetter)]
pub trait HasReference<'a, T: 'a + ?Sized> {
    fn get_reference(&self) -> &'a T;
}

#[cgp_impl(new GetName)]
#[uses(HasField<Symbol!("name"), Value = &'a str>)]
impl<'a> ReferenceGetter<'a, str> {
    fn get_reference(&self) -> &'a str {
        self.get_field(PhantomData::<Symbol!("name")>)
    }
}

#[derive(HasField)]
pub struct Borrowed<'a> {
    pub name: &'a str,
}

delegate_components! {
    <'a> Borrowed<'a> {
        ReferenceGetterComponent: GetName,
    }
}

check_components! {
    <'a> Borrowed<'a> {
        ReferenceGetterComponent: (Life<'a>, str),
    }
}

// Generic code bounded on the consumer trait at the unsized argument.
fn read<'a, Context: HasReference<'a, str>>(context: &Context) -> &'a str {
    context.get_reference()
}

#[test]
fn test_unsized_parameter_resolves_through_the_table() {
    let text = String::from("hello");
    let borrowed = Borrowed { name: &text };

    let name: &str = borrowed.get_reference();
    assert_eq!(name, "hello");
    assert_eq!(read(&borrowed), "hello");
}
