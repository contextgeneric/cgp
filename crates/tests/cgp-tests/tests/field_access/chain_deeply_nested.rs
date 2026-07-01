//! Deeply nested field access: five owned structs (`MyContext.a.b.c.d.name`),
//! each deriving `HasField`, reached in one hop by wiring a `#[cgp_getter]` to
//! `WithProvider<ChainGetters<Product![UseField<..>, ..]>>`.
//!
//! This concept owns the `#[derive(HasField)]` snapshots, so every derive here
//! keeps its snapshot. The `#[cgp_getter]` trait and the
//! `delegate_and_check_components!` wiring are incidental scaffolding — their
//! expansions are owned by the `getters` and `checking` targets — so they are
//! written as the plain macros.
//!
//! See docs/reference/derives/derive_has_field.md and
//! docs/reference/traits/has_field.md.

use cgp::core::field::impls::ChainGetters;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_field;

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct A {
        pub b: B,
    }

    expand_a(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<1, Chars<'b', Nil>>> for A {
            type Value = B;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'b', Nil>>>,
            ) -> &Self::Value {
                &self.b
            }
        }
        impl HasFieldMut<Symbol<1, Chars<'b', Nil>>> for A {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'b', Nil>>>,
            ) -> &mut Self::Value {
                &mut self.b
            }
        }
        ")
    }
}

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct B {
        pub c: C,
    }

    expand_b(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<1, Chars<'c', Nil>>> for B {
            type Value = C;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'c', Nil>>>,
            ) -> &Self::Value {
                &self.c
            }
        }
        impl HasFieldMut<Symbol<1, Chars<'c', Nil>>> for B {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'c', Nil>>>,
            ) -> &mut Self::Value {
                &mut self.c
            }
        }
        ")
    }
}

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct C {
        pub d: D,
    }

    expand_c(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<1, Chars<'d', Nil>>> for C {
            type Value = D;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'d', Nil>>>,
            ) -> &Self::Value {
                &self.d
            }
        }
        impl HasFieldMut<Symbol<1, Chars<'d', Nil>>> for C {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'d', Nil>>>,
            ) -> &mut Self::Value {
                &mut self.d
            }
        }
        ")
    }
}

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct D {
        pub name: String,
    }

    expand_d(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for D {
            type Value = String;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                >,
            ) -> &Self::Value {
                &self.name
            }
        }
        impl HasFieldMut<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>> for D {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                >,
            ) -> &mut Self::Value {
                &mut self.name
            }
        }
        ")
    }
}

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct MyContext {
        pub a: A,
    }

    expand_my_context_struct(output) {
        insta::assert_snapshot!(output, @"
        impl HasField<Symbol<1, Chars<'a', Nil>>> for MyContext {
            type Value = A;
            fn get_field(
                &self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'a', Nil>>>,
            ) -> &Self::Value {
                &self.a
            }
        }
        impl HasFieldMut<Symbol<1, Chars<'a', Nil>>> for MyContext {
            fn get_field_mut(
                &mut self,
                key: ::core::marker::PhantomData<Symbol<1, Chars<'a', Nil>>>,
            ) -> &mut Self::Value {
                &mut self.a
            }
        }
        ")
    }
}

// Incidental scaffolding: the `#[cgp_getter]` expansion is owned by the
// `getters` target, so it is written plainly here.
#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

// Incidental scaffolding: the `delegate_and_check_components!` expansion is owned
// by the `checking` target, so it is written plainly here.
delegate_and_check_components! {
    MyContext {
        NameGetterComponent: WithProvider<
            ChainGetters<Product![
                UseField<Symbol!("a")>,
                UseField<Symbol!("b")>,
                UseField<Symbol!("c")>,
                UseField<Symbol!("d")>,
                UseField<Symbol!("name")>
            ]>>
    }
}

#[test]
fn test_deeply_nested_getter() {
    let context = MyContext {
        a: A {
            b: B {
                c: C {
                    d: D {
                        name: "test".to_owned(),
                    },
                },
            },
        },
    };

    assert_eq!(context.name(), "test");
}
