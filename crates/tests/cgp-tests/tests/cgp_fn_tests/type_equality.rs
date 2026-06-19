use std::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;
use insta::assert_snapshot;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasScalarType::{Scalar = f64})]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        let res: f64 = width * height;
        res
    }

    expand_rectangle_area(output) {
        assert_snapshot!(output, @"
        pub trait RectangleArea: HasScalarType {
            fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = <Self as HasScalarType>::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = <Self as HasScalarType>::Scalar,
                >,
            Self: HasScalarType<Scalar = f64>,
        {
            fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar {
                let width: <Self as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: <Self as HasScalarType>::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'h',
                                    Chars<
                                        'e',
                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone();
                let res: f64 = width * height;
                res
            }
        }
        ")
    }
}

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
        assert_snapshot!(output, @"
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
        assert_snapshot!(output, @"
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
        assert_snapshot!(output, @"
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
