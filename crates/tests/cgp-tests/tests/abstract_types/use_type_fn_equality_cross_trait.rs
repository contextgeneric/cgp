//! `#[use_type]` type-equality *across two traits* in `#[cgp_fn]`:
//! `#[use_type(HasBarType::{Bar as Baz = Foo}, HasFooType::Foo)]`.
//!
//! The `{Bar as Baz = Foo}` form imports `HasBarType::Bar` under the alias `Baz`
//! and equates it to the `Foo` alias imported from `HasFooType`, generating a
//! `Self: HasBarType<Bar = <Self as HasFooType>::Foo>` bound. The consequence
//! pinned here: the `Ord + Clone` bounds declared on `HasFooType::Foo` become
//! visible for both aliases, while the `Display` bound on `HasBarType::Bar` is
//! hidden — because the two are the same type. `do_foo`/`do_bar` are the
//! dependencies `return_foo_or_bar` imports with `#[uses]`. All three `#[cgp_fn]`
//! snapshots are kept because the abstract-type rewrite is the point.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use std::fmt::Display;

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait HasFooType {
    // The `Ord + Clone` bounds are visible to both `Foo` and `Bar` because of `Bar = Foo` below
    type Foo: Ord + Clone;
}

pub trait HasBarType {
    // The `Display` bounds are hidden because of `Bar = Foo` below
    type Bar: Display;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasFooType::Foo)]
    pub fn do_foo(&self) -> Foo {
        todo!()
    }

    expand_do_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait DoFoo: HasFooType {
            fn do_foo(&self) -> <Self as HasFooType>::Foo;
        }
        impl<__Context__> DoFoo for __Context__
        where
            Self: HasFooType,
        {
            fn do_foo(&self) -> <Self as HasFooType>::Foo {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasBarType::Bar)]
    pub fn do_bar(&self) -> Bar {
        todo!()
    }

    expand_do_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait DoBar: HasBarType {
            fn do_bar(&self) -> <Self as HasBarType>::Bar;
        }
        impl<__Context__> DoBar for __Context__
        where
            Self: HasBarType,
        {
            fn do_bar(&self) -> <Self as HasBarType>::Bar {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasBarType::{Bar as Baz = Foo}, HasFooType::Foo)]
    #[uses(DoFoo, DoBar)]
    fn return_foo_or_bar(&self, flag: bool, #[implicit] foo: &Foo, #[implicit] bar: &Baz) -> Foo {
        if flag {
            let res: Foo = self.do_foo();
            if &res < foo { res } else { foo.clone() }
        } else {
            let res: Baz = self.do_bar();
            if &res < bar { res } else { bar.clone() }
        }
    }

    expand_return_foo_or_bar(output) {
        insta::assert_snapshot!(output, @"
        trait ReturnFooOrBar: HasBarType + HasFooType {
            fn return_foo_or_bar(&self, flag: bool) -> <Self as HasFooType>::Foo;
        }
        impl<__Context__> ReturnFooOrBar for __Context__
        where
            Self: DoFoo + DoBar,
            Self: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = <Self as HasFooType>::Foo,
                >
                + HasField<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    Value = <Self as HasBarType>::Bar,
                >,
            Self: HasBarType<Bar = <Self as HasFooType>::Foo>,
            Self: HasFooType,
        {
            fn return_foo_or_bar(&self, flag: bool) -> <Self as HasFooType>::Foo {
                let foo: &<Self as HasFooType>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    );
                let bar: &<Self as HasBarType>::Bar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                        >,
                    );
                if flag {
                    let res: <Self as HasFooType>::Foo = self.do_foo();
                    if &res < foo { res } else { foo.clone() }
                } else {
                    let res: <Self as HasBarType>::Bar = self.do_bar();
                    if &res < bar { res } else { bar.clone() }
                }
            }
        }
        ")
    }
}
