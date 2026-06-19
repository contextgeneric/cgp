use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    #[impl_generics(Name: Display)]
    pub fn greet(&self, #[implicit] name: &Name) -> String
    where
        Name: Display,
    {
        format!("Hello, {}!", name)
    }

    expand_greet(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait Greet {
            fn greet(&self) -> String;
        }
        impl<__Context__, Name: Display> Greet for __Context__
        where
            Name: Display,
            Self: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = Name,
            >,
        {
            fn greet(&self) -> String {
                let name: &Name = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    );
                format!("Hello, {}!", name)
            }
        }
        "#)
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[uses(Greet)]
    pub fn test_greet(&self) {
        assert_eq!(self.greet(), "Hello, John!");
    }

    expand_test_greet(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait TestGreet {
            fn test_greet(&self);
        }
        impl<__Context__> TestGreet for __Context__
        where
            Self: Greet,
        {
            fn test_greet(&self) {
                assert_eq!(self.greet(), "Hello, John!");
            }
        }
        "#)
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

#[test]
fn test_impl_generics() {
    let person = Person {
        name: "John".to_string(),
    };

    person.test_greet();
}
