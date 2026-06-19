use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

pub trait HasFooType<T> {
    type Foo;
}

pub trait HasBarType {
    type Bar;

    type Baz;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[allow(unused)]
    #[async_trait]
    #[use_type(<HasFooType<X>>::{Foo as FooX}, <HasFooType<Y>>::{Foo as FooY}, HasBarType::{Bar, Baz})]
    pub async fn do_foo_bar<X, Y>(
        &self,
        x: X,
        #[implicit] foo_x: &FooX,
        #[implicit] foo_y: &FooY,
        #[implicit] bar: &Bar,
        y: Y,
    ) -> Option<Baz>
    where
        FooX: Display,
    {
        None
    }

    expand_do_foo_bar(output) {
        insta::assert_snapshot!(output, @"
        #[allow(unused)]
        #[async_trait]
        pub trait DoFooBar<X, Y>: HasFooType<X> + HasFooType<Y> + HasBarType {
            async fn do_foo_bar(&self, x: X, y: Y) -> Option<<Self as HasBarType>::Baz>;
        }
        #[allow(unused)]
        #[async_trait]
        impl<__Context__, X, Y> DoFooBar<X, Y> for __Context__
        where
            <Self as HasFooType<X>>::Foo: Display,
            Self: HasField<
                    Symbol<5, Chars<'f', Chars<'o', Chars<'o', Chars<'_', Chars<'x', Nil>>>>>>,
                    Value = <Self as HasFooType<X>>::Foo,
                >
                + HasField<
                    Symbol<5, Chars<'f', Chars<'o', Chars<'o', Chars<'_', Chars<'y', Nil>>>>>>,
                    Value = <Self as HasFooType<Y>>::Foo,
                >
                + HasField<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    Value = <Self as HasBarType>::Bar,
                >,
            Self: HasFooType<X>,
            Self: HasFooType<Y>,
            Self: HasBarType,
        {
            async fn do_foo_bar(&self, x: X, y: Y) -> Option<<Self as HasBarType>::Baz> {
                let foo_x: &<Self as HasFooType<X>>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'f', Chars<'o', Chars<'o', Chars<'_', Chars<'x', Nil>>>>>,
                            >,
                        >,
                    );
                let foo_y: &<Self as HasFooType<Y>>::Foo = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'f', Chars<'o', Chars<'o', Chars<'_', Chars<'y', Nil>>>>>,
                            >,
                        >,
                    );
                let bar: &<Self as HasBarType>::Bar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                        >,
                    );
                None
            }
        }
        ")
    }
}
