use core::marker::PhantomData;

use cgp::prelude::*;

#[test]
fn test_context_with_lifetime_field() {
    #[derive(HasField)]
    pub struct Context<'a> {
        pub name: &'a str,
    }

    let context = Context { name: "test" };

    assert_eq!(context.get_field(PhantomData), &"test");
}
