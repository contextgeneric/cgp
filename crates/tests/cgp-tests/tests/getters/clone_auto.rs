//! `#[cgp_auto_getter]` returning an owned `Self::Name` where the associated type
//! is `Copy`: the blanket impl reads the field named after the method and
//! `.clone()`s it out by value. The abstract `HasNameType` and the
//! `delegate_components!` wiring are written plainly here (their expansions are
//! owned by the `abstract_types` and `basic_delegation` concepts).
//!
//! See docs/reference/macros/cgp_auto_getter.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_getter;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    pub trait HasName: HasNameType<Name: Copy> {
        fn name(&self) -> Self::Name;
    }

    expand_has_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasName: HasNameType<Name: Copy> {
            fn name(&self) -> Self::Name;
        }
        impl<__Context__> HasName for __Context__
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = __Context__::Name,
            >,
        {
            fn name(&self) -> __Context__::Name {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .clone()
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct App {
    pub name: &'static str,
}

delegate_components! {
    App {
        NameTypeProviderComponent: UseType<&'static str>,
    }
}

#[test]
pub fn test_clone_auto_getter() {
    let context = App { name: "Alice" };

    assert_eq!(context.name(), "Alice");
}
