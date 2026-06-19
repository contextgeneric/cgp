use std::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_fn, snapshot_cgp_type};

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasScalarType {
        type Scalar;
    }

    expand_has_scalar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalarType {
            type Scalar;
        }
        impl<__Context__> HasScalarType for __Context__
        where
            __Context__: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <__Context__ as ScalarTypeProvider<__Context__>>::Scalar;
        }
        pub trait ScalarTypeProvider<
            __Context__,
        >: IsProviderFor<ScalarTypeProviderComponent, __Context__, ()> {
            type Scalar;
        }
        impl<__Provider__, __Context__> ScalarTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ScalarTypeProviderComponent>
                + IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ScalarTypeProviderComponent,
            >>::Delegate: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <<__Provider__ as DelegateComponent<
                ScalarTypeProviderComponent,
            >>::Delegate as ScalarTypeProvider<__Context__>>::Scalar;
        }
        pub struct ScalarTypeProviderComponent;
        impl<__Context__> ScalarTypeProvider<__Context__> for UseContext
        where
            __Context__: HasScalarType,
        {
            type Scalar = <__Context__ as HasScalarType>::Scalar;
        }
        impl<__Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseContext
        where
            __Context__: HasScalarType,
        {}
        impl<__Context__, __Components__, __Path__> ScalarTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as ScalarTypeProvider<__Context__>>::Scalar;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
                + ScalarTypeProvider<__Context__>,
        {}
        impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar>
        where
            Scalar:,
        {
            type Scalar = Scalar;
        }
        impl<Scalar, __Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseType<Scalar>
        where
            Scalar:,
        {}
        impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
            Scalar:,
        {
            type Scalar = Scalar;
        }
        impl<
            __Provider__,
            Scalar,
            __Context__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
            Scalar:,
        {}
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasScalarType::{Scalar = f64})]
    pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        let res: f64 = width * height;
        res
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
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
