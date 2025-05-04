use core::marker::PhantomData;

use cgp::core::field::ChainGetters;
use cgp::prelude::*;

#[test]
fn test_chained_getter() {
    #[derive(HasField)]
    pub struct Outer {
        pub inner: Inner,
    }

    #[derive(HasField)]
    pub struct Inner {
        pub name: String,
    }

    let context = Outer {
        inner: Inner {
            name: "test".to_owned(),
        },
    };

    let name: &String =
        <ChainGetters<UseField<symbol!("inner")>, UseField<symbol!("name")>>>::get_field(
            &context,
            PhantomData::<()>,
        );
    assert_eq!(name, "test");
}
