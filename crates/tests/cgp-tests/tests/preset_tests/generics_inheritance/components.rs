use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;
use insta::assert_snapshot;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

snapshot_cgp_getter! {
    #[cgp_getter {
        name: FooGetterComponent<I>,
        provider: FooGetter,
    }]
    pub trait HasFooAt<I>: HasFooType {
        fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
    }

    expand_has_foo_at(output) {
        assert_snapshot!(output, @"
        pub trait HasFooAt<I>: HasFooType {
            fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
        }
        impl<__Context__, I> HasFooAt<I> for __Context__
        where
            __Context__: HasFooType,
            __Context__: FooGetter<__Context__, I>,
        {
            fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo {
                __Context__::foo(self, _tag)
            }
        }
        pub trait FooGetter<
            __Context__,
            I,
        >: IsProviderFor<FooGetterComponent<I>, __Context__, (I)>
        where
            __Context__: HasFooType,
        {
            fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo;
        }
        impl<__Provider__, __Context__, I> FooGetter<__Context__, I> for __Provider__
        where
            __Context__: HasFooType,
            __Provider__: DelegateComponent<FooGetterComponent<I>>
                + IsProviderFor<FooGetterComponent<I>, __Context__, (I)>,
            <__Provider__ as DelegateComponent<
                FooGetterComponent<I>,
            >>::Delegate: FooGetter<__Context__, I>,
        {
            fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                <__Provider__ as DelegateComponent<
                    FooGetterComponent<I>,
                >>::Delegate::foo(__context__, _tag)
            }
        }
        pub struct FooGetterComponent<I>(pub ::core::marker::PhantomData<(I)>);
        impl<__Context__, I> FooGetter<__Context__, I> for UseContext
        where
            __Context__: HasFooType,
            __Context__: HasFooAt<I>,
        {
            fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                __Context__::foo(__context__, _tag)
            }
        }
        impl<__Context__, I> IsProviderFor<FooGetterComponent<I>, __Context__, (I)>
        for UseContext
        where
            __Context__: HasFooType,
            __Context__: HasFooAt<I>,
        {}
        impl<__Context__, I, __Components__, __Path__> FooGetter<__Context__, I>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType,
            __Path__: ConcatPath<PathCons<I, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >>::Delegate: FooGetter<__Context__, I>,
        {
            fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >>::Delegate::foo(__context__, _tag)
            }
        }
        impl<
            __Context__,
            I,
            __Components__,
            __Path__,
        > IsProviderFor<FooGetterComponent<I>, __Context__, (I)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooType,
            __Path__: ConcatPath<PathCons<I, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >>::Delegate: IsProviderFor<FooGetterComponent<I>, __Context__, (I)>
                + FooGetter<__Context__, I>,
        {}
        impl<__Context__, I> FooGetter<__Context__, I> for UseFields
        where
            __Context__: HasFooType,
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = __Context__::Foo,
            >,
        {
            fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                        >,
                    )
            }
        }
        impl<__Context__, I> IsProviderFor<FooGetterComponent<I>, __Context__, (I)> for UseFields
        where
            __Context__: HasFooType,
            __Context__: HasField<
                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                Value = __Context__::Foo,
            >,
        {}
        impl<__Context__, I, __Tag__> FooGetter<__Context__, I> for UseField<__Tag__>
        where
            __Context__: HasFooType,
            __Context__: HasField<__Tag__, Value = __Context__::Foo>,
        {
            fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, I, __Tag__> IsProviderFor<FooGetterComponent<I>, __Context__, (I)>
        for UseField<__Tag__>
        where
            __Context__: HasFooType,
            __Context__: HasField<__Tag__, Value = __Context__::Foo>,
        {}
        impl<__Context__, I, __Provider__> FooGetter<__Context__, I>
        for WithProvider<__Provider__>
        where
            __Context__: HasFooType,
            __Provider__: FieldGetter<
                __Context__,
                FooGetterComponent<I>,
                Value = __Context__::Foo,
            >,
        {
            fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<FooGetterComponent<I>>,
                )
            }
        }
        impl<__Context__, I, __Provider__> IsProviderFor<FooGetterComponent<I>, __Context__, (I)>
        for WithProvider<__Provider__>
        where
            __Context__: HasFooType,
            __Provider__: FieldGetter<
                __Context__,
                FooGetterComponent<I>,
                Value = __Context__::Foo,
            >,
        {}
        ")
    }
}

snapshot_cgp_getter! {
    #[cgp_getter {
        name: BarGetterComponent<I>,
        provider: BarGetter,
    }]
    pub trait HasBarAt<I>: HasBarType {
        fn bar(&self) -> &Self::Bar;
    }

    expand_has_bar_at(output) {
        assert_snapshot!(output, @"
        pub trait HasBarAt<I>: HasBarType {
            fn bar(&self) -> &Self::Bar;
        }
        impl<__Context__, I> HasBarAt<I> for __Context__
        where
            __Context__: HasBarType,
            __Context__: BarGetter<__Context__, I>,
        {
            fn bar(&self) -> &Self::Bar {
                __Context__::bar(self)
            }
        }
        pub trait BarGetter<
            __Context__,
            I,
        >: IsProviderFor<BarGetterComponent<I>, __Context__, (I)>
        where
            __Context__: HasBarType,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar;
        }
        impl<__Provider__, __Context__, I> BarGetter<__Context__, I> for __Provider__
        where
            __Context__: HasBarType,
            __Provider__: DelegateComponent<BarGetterComponent<I>>
                + IsProviderFor<BarGetterComponent<I>, __Context__, (I)>,
            <__Provider__ as DelegateComponent<
                BarGetterComponent<I>,
            >>::Delegate: BarGetter<__Context__, I>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                <__Provider__ as DelegateComponent<
                    BarGetterComponent<I>,
                >>::Delegate::bar(__context__)
            }
        }
        pub struct BarGetterComponent<I>(pub ::core::marker::PhantomData<(I)>);
        impl<__Context__, I> BarGetter<__Context__, I> for UseContext
        where
            __Context__: HasBarType,
            __Context__: HasBarAt<I>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __Context__::bar(__context__)
            }
        }
        impl<__Context__, I> IsProviderFor<BarGetterComponent<I>, __Context__, (I)>
        for UseContext
        where
            __Context__: HasBarType,
            __Context__: HasBarAt<I>,
        {}
        impl<__Context__, I, __Components__, __Path__> BarGetter<__Context__, I>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasBarType,
            __Path__: ConcatPath<PathCons<I, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >>::Delegate: BarGetter<__Context__, I>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >>::Delegate::bar(__context__)
            }
        }
        impl<
            __Context__,
            I,
            __Components__,
            __Path__,
        > IsProviderFor<BarGetterComponent<I>, __Context__, (I)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasBarType,
            __Path__: ConcatPath<PathCons<I, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
            >>::Delegate: IsProviderFor<BarGetterComponent<I>, __Context__, (I)>
                + BarGetter<__Context__, I>,
        {}
        impl<__Context__, I> BarGetter<__Context__, I> for UseFields
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
        impl<__Context__, I> IsProviderFor<BarGetterComponent<I>, __Context__, (I)> for UseFields
        where
            __Context__: HasBarType,
            __Context__: HasField<
                Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                Value = __Context__::Bar,
            >,
        {}
        impl<__Context__, I, __Tag__> BarGetter<__Context__, I> for UseField<__Tag__>
        where
            __Context__: HasBarType,
            __Context__: HasField<__Tag__, Value = __Context__::Bar>,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, I, __Tag__> IsProviderFor<BarGetterComponent<I>, __Context__, (I)>
        for UseField<__Tag__>
        where
            __Context__: HasBarType,
            __Context__: HasField<__Tag__, Value = __Context__::Bar>,
        {}
        impl<__Context__, I, __Provider__> BarGetter<__Context__, I>
        for WithProvider<__Provider__>
        where
            __Context__: HasBarType,
            __Provider__: FieldGetter<
                __Context__,
                BarGetterComponent<I>,
                Value = __Context__::Bar,
            >,
        {
            fn bar(__context__: &__Context__) -> &__Context__::Bar {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<BarGetterComponent<I>>,
                )
            }
        }
        impl<__Context__, I, __Provider__> IsProviderFor<BarGetterComponent<I>, __Context__, (I)>
        for WithProvider<__Provider__>
        where
            __Context__: HasBarType,
            __Provider__: FieldGetter<
                __Context__,
                BarGetterComponent<I>,
                Value = __Context__::Bar,
            >,
        {}
        ")
    }
}
