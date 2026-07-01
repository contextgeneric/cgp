//! `#[use_type]` reaching through a *nested foreign* associated type across traits
//! in `#[cgp_fn]`: `#[use_type(HasBarType::Bar, @Bar::HasFooType::Foo)]` and the
//! equality form `@Bar::HasFooType::{Foo as BarFoo = Foo}`.
//!
//! Here `HasBarType::Bar` itself implements `HasFooType`, so `@Bar::HasFooType::Foo`
//! resolves the alias to the two-hop `<<Self as HasBarType>::Bar as HasFooType>::Foo`
//! and adds `<Self as HasBarType>::Bar: HasFooType` to the impl. The final function
//! equates that nested type to `Self`'s own `Foo` (`{Foo as BarFoo = Foo}`),
//! generating a `HasFooType<Foo = <Self as HasFooType>::Foo>` bound on the nested
//! type. `do_foo`/`do_bar` are the dependencies imported by `return_foo_or_bar`.
//! All three `#[cgp_fn]` snapshots are kept because the abstract-type rewrite is
//! the point.
//!
//! See docs/reference/attributes/use_type.md and docs/concepts/abstract-types.md.

use cgp_macro_test_util::snapshot_cgp_fn;

pub trait HasFooType {
    type Foo: Ord + Clone;
}

pub trait HasBarType {
    type Bar: HasFooType;
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
    #[use_type(HasBarType::Bar, @Bar::HasFooType::Foo)]
    pub fn do_bar(&self) -> Foo {
        todo!()
    }

    expand_do_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait DoBar: HasBarType {
            fn do_bar(&self) -> <<Self as HasBarType>::Bar as HasFooType>::Foo;
        }
        impl<__Context__> DoBar for __Context__
        where
            Self: HasBarType,
            <Self as HasBarType>::Bar: HasFooType,
        {
            fn do_bar(&self) -> <<Self as HasBarType>::Bar as HasFooType>::Foo {
                todo!()
            }
        }
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(
        HasFooType::Foo,
        HasBarType::Bar,
        @Bar::HasFooType::{Foo as BarFoo = Foo},
    )]
    #[uses(DoFoo, DoBar)]
    fn return_foo_or_bar(&self, flag: bool, #[implicit] foo: &Foo, #[implicit] bar: &BarFoo) -> Foo {
        if flag {
            let res: Foo = self.do_foo();
            if &res < foo { res } else { foo.clone() }
        } else {
            let res: BarFoo = self.do_bar();
            if &res < bar { res } else { bar.clone() }
        }
    }

    expand_return_foo_or_bar(output) {
        insta::assert_snapshot!(output, @"
        trait ReturnFooOrBar: HasFooType + HasBarType {
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
                    Value = <<Self as HasBarType>::Bar as HasFooType>::Foo,
                >,
            Self: HasFooType,
            Self: HasBarType,
            <Self as HasBarType>::Bar: HasFooType<Foo = <Self as HasFooType>::Foo>,
        {
            fn return_foo_or_bar(&self, flag: bool) -> <Self as HasFooType>::Foo {
                let foo: &<Self as HasFooType>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    );
                let bar: &<<Self as HasBarType>::Bar as HasFooType>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                        >,
                    );
                if flag {
                    let res: <Self as HasFooType>::Foo = self.do_foo();
                    if &res < foo { res } else { foo.clone() }
                } else {
                    let res: <<Self as HasBarType>::Bar as HasFooType>::Foo = self.do_bar();
                    if &res < bar { res } else { bar.clone() }
                }
            }
        }
        ")
    }
}
