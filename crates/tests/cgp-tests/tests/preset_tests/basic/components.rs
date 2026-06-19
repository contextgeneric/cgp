use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_cgp_type};

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasFooType {
        type Foo;
    }

    expand_has_foo_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooType {
            type Foo;
        }
        impl<__Context__> HasFooType for __Context__
        where
            __Context__: FooTypeProvider<__Context__>,
        {
            type Foo = <__Context__ as FooTypeProvider<__Context__>>::Foo;
        }
        pub trait FooTypeProvider<
            __Context__,
        >: IsProviderFor<FooTypeProviderComponent, __Context__, ()> {
            type Foo;
        }
        impl<__Provider__, __Context__> FooTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FooTypeProviderComponent>
                + IsProviderFor<FooTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooTypeProviderComponent,
            >>::Delegate: FooTypeProvider<__Context__>,
        {
            type Foo = <<__Provider__ as DelegateComponent<
                FooTypeProviderComponent,
            >>::Delegate as FooTypeProvider<__Context__>>::Foo;
        }
        pub struct FooTypeProviderComponent;
        impl<__Context__> FooTypeProvider<__Context__> for UseContext
        where
            __Context__: HasFooType,
        {
            type Foo = <__Context__ as HasFooType>::Foo;
        }
        impl<__Context__> IsProviderFor<FooTypeProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType,
        {}
        impl<__Context__, __Components__, __Path__> FooTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: FooTypeProvider<__Context__>,
        {
            type Foo = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as FooTypeProvider<__Context__>>::Foo;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooTypeProviderComponent, __Context__, ()>
                + FooTypeProvider<__Context__>,
        {}
        impl<Foo, __Context__> FooTypeProvider<__Context__> for UseType<Foo>
        where
            Foo:,
        {
            type Foo = Foo;
        }
        impl<Foo, __Context__> IsProviderFor<FooTypeProviderComponent, __Context__, ()>
        for UseType<Foo>
        where
            Foo:,
        {}
        impl<__Provider__, Foo, __Context__> FooTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, FooTypeProviderComponent, Type = Foo>,
            Foo:,
        {
            type Foo = Foo;
        }
        impl<
            __Provider__,
            Foo,
            __Context__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, ()> for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, FooTypeProviderComponent, Type = Foo>,
            Foo:,
        {}
        ")
    }
}

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasBarType {
        type Bar;
    }

    expand_has_bar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasBarType {
            type Bar;
        }
        impl<__Context__> HasBarType for __Context__
        where
            __Context__: BarTypeProvider<__Context__>,
        {
            type Bar = <__Context__ as BarTypeProvider<__Context__>>::Bar;
        }
        pub trait BarTypeProvider<
            __Context__,
        >: IsProviderFor<BarTypeProviderComponent, __Context__, ()> {
            type Bar;
        }
        impl<__Provider__, __Context__> BarTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<BarTypeProviderComponent>
                + IsProviderFor<BarTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                BarTypeProviderComponent,
            >>::Delegate: BarTypeProvider<__Context__>,
        {
            type Bar = <<__Provider__ as DelegateComponent<
                BarTypeProviderComponent,
            >>::Delegate as BarTypeProvider<__Context__>>::Bar;
        }
        pub struct BarTypeProviderComponent;
        impl<__Context__> BarTypeProvider<__Context__> for UseContext
        where
            __Context__: HasBarType,
        {
            type Bar = <__Context__ as HasBarType>::Bar;
        }
        impl<__Context__> IsProviderFor<BarTypeProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: HasBarType,
        {}
        impl<__Context__, __Components__, __Path__> BarTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: BarTypeProvider<__Context__>,
        {
            type Bar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as BarTypeProvider<__Context__>>::Bar;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<BarTypeProviderComponent, __Context__, ()>
                + BarTypeProvider<__Context__>,
        {}
        impl<Bar, __Context__> BarTypeProvider<__Context__> for UseType<Bar>
        where
            Bar:,
        {
            type Bar = Bar;
        }
        impl<Bar, __Context__> IsProviderFor<BarTypeProviderComponent, __Context__, ()>
        for UseType<Bar>
        where
            Bar:,
        {}
        impl<__Provider__, Bar, __Context__> BarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, BarTypeProviderComponent, Type = Bar>,
            Bar:,
        {
            type Bar = Bar;
        }
        impl<
            __Provider__,
            Bar,
            __Context__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, ()> for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, BarTypeProviderComponent, Type = Bar>,
            Bar:,
        {}
        ")
    }
}

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasFoo: HasFooType {
        fn foo(&self) -> &Self::Foo;
    }

    expand_has_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFoo: HasFooType {
            fn foo(&self) -> &Self::Foo;
        }
        impl<__Context__> HasFoo for __Context__
        where
            __Context__: HasFooType,
            __Context__: FooGetter<__Context__>,
        {
            fn foo(&self) -> &Self::Foo {
                __Context__::foo(self)
            }
        }
        pub trait FooGetter<__Context__>: IsProviderFor<FooGetterComponent, __Context__, ()>
        where
            __Context__: HasFooType,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo;
        }
        impl<__Provider__, __Context__> FooGetter<__Context__> for __Provider__
        where
            __Context__: HasFooType,
            __Provider__: DelegateComponent<FooGetterComponent>
                + IsProviderFor<FooGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooGetterComponent,
            >>::Delegate: FooGetter<__Context__>,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo {
                <__Provider__ as DelegateComponent<
                    FooGetterComponent,
                >>::Delegate::foo(__context__)
            }
        }
        pub struct FooGetterComponent;
        impl<__Context__> FooGetter<__Context__> for UseContext
        where
            __Context__: HasFooType,
            __Context__: HasFoo,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo {
                __Context__::foo(__context__)
            }
        }
        impl<__Context__> IsProviderFor<FooGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasFooType,
            __Context__: HasFoo,
        {}
        impl<__Context__, __Components__, __Path__> FooGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooGetter<__Context__>,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::foo(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooGetterComponent, __Context__, ()>
                + FooGetter<__Context__>,
        {}
        impl<__Context__> FooGetter<__Context__> for UseFields
        where
            __Context__: HasFooType,
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = __Context__::Foo,
            >,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
            }
        }
        impl<__Context__> IsProviderFor<FooGetterComponent, __Context__, ()> for UseFields
        where
            __Context__: HasFooType,
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = __Context__::Foo,
            >,
        {}
        impl<__Context__, __Tag__> FooGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasFooType,
            __Context__: HasField<__Tag__, Value = __Context__::Foo>,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<FooGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasFooType,
            __Context__: HasField<__Tag__, Value = __Context__::Foo>,
        {}
        impl<__Context__, __Provider__> FooGetter<__Context__> for WithProvider<__Provider__>
        where
            __Context__: HasFooType,
            __Provider__: FieldGetter<__Context__, FooGetterComponent, Value = __Context__::Foo>,
        {
            fn foo(__context__: &__Context__) -> &__Context__::Foo {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<FooGetterComponent>,
                )
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<FooGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Context__: HasFooType,
            __Provider__: FieldGetter<__Context__, FooGetterComponent, Value = __Context__::Foo>,
        {}
        ")
    }
}

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasBar: HasBarType {
        fn bar(&self) -> &Self::Bar;
    }

    expand_has_bar(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasBar: HasBarType {
            fn bar(&self) -> &Self::Bar;
        }
        impl<__Context__> HasBar for __Context__
        where
            __Context__: HasBarType,
            __Context__: BarGetter<__Context__>,
        {
            fn bar(&self) -> &Self::Bar {
                __Context__::bar(self)
            }
        }
        pub trait BarGetter<__Context__>: IsProviderFor<BarGetterComponent, __Context__, ()>
        where
            __Context__: HasBarType,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar;
        }
        impl<__Provider__, __Context__> BarGetter<__Context__> for __Provider__
        where
            __Context__: HasBarType,
            __Provider__: DelegateComponent<BarGetterComponent>
                + IsProviderFor<BarGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                BarGetterComponent,
            >>::Delegate: BarGetter<__Context__>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                <__Provider__ as DelegateComponent<
                    BarGetterComponent,
                >>::Delegate::bar(__context__)
            }
        }
        pub struct BarGetterComponent;
        impl<__Context__> BarGetter<__Context__> for UseContext
        where
            __Context__: HasBarType,
            __Context__: HasBar,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __Context__::bar(__context__)
            }
        }
        impl<__Context__> IsProviderFor<BarGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasBarType,
            __Context__: HasBar,
        {}
        impl<__Context__, __Components__, __Path__> BarGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasBarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: BarGetter<__Context__>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::bar(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<BarGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasBarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<BarGetterComponent, __Context__, ()>
                + BarGetter<__Context__>,
        {}
        impl<__Context__> BarGetter<__Context__> for UseFields
        where
            __Context__: HasBarType,
            __Context__: HasField<
                Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                Value = __Context__::Bar,
            >,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                        >,
                    )
            }
        }
        impl<__Context__> IsProviderFor<BarGetterComponent, __Context__, ()> for UseFields
        where
            __Context__: HasBarType,
            __Context__: HasField<
                Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                Value = __Context__::Bar,
            >,
        {}
        impl<__Context__, __Tag__> BarGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasBarType,
            __Context__: HasField<__Tag__, Value = __Context__::Bar>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<BarGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasBarType,
            __Context__: HasField<__Tag__, Value = __Context__::Bar>,
        {}
        impl<__Context__, __Provider__> BarGetter<__Context__> for WithProvider<__Provider__>
        where
            __Context__: HasBarType,
            __Provider__: FieldGetter<__Context__, BarGetterComponent, Value = __Context__::Bar>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<BarGetterComponent>,
                )
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<BarGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Context__: HasBarType,
            __Provider__: FieldGetter<__Context__, BarGetterComponent, Value = __Context__::Bar>,
        {}
        ")
    }
}
