use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;
use insta::assert_snapshot;

pub struct UseDelegate2<Components>(pub PhantomData<Components>);

#[cgp_type {
    provider: FooTypeProviderAt,
    derive_delegate: [
        UseDelegate<I>,
        UseDelegate2<(I, J)>,
    ],
}]
pub trait HasFooTypeAt<I, J> {
    type Foo;
}

snapshot_cgp_getter! {
    #[cgp_getter {
        provider: FooGetterAt,
        derive_delegate: [
            UseDelegate<I>,
            UseDelegate2<(I, J)>,
        ],
    }]
    pub trait HasFooAt<I, J>: HasFooTypeAt<I, J> {
        fn foo_at(&self, _tag: PhantomData<(I, J)>) -> &Self::Foo;
    }

    expand_has_foo_at(output) {
        assert_snapshot!(output, @"
        pub trait HasFooAt<I, J>: HasFooTypeAt<I, J> {
            fn foo_at(&self, _tag: PhantomData<(I, J)>) -> &Self::Foo;
        }
        impl<__Context__, I, J> HasFooAt<I, J> for __Context__
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: FooGetterAt<__Context__, I, J>,
        {
            fn foo_at(&self, _tag: PhantomData<(I, J)>) -> &Self::Foo {
                __Context__::foo_at(self, _tag)
            }
        }
        pub trait FooGetterAt<
            __Context__,
            I,
            J,
        >: IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        where
            __Context__: HasFooTypeAt<I, J>,
        {
            fn foo_at(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Foo;
        }
        impl<__Provider__, __Context__, I, J> FooGetterAt<__Context__, I, J> for __Provider__
        where
            __Context__: HasFooTypeAt<I, J>,
            __Provider__: DelegateComponent<FooGetterAtComponent>
                + IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>,
            <__Provider__ as DelegateComponent<
                FooGetterAtComponent,
            >>::Delegate: FooGetterAt<__Context__, I, J>,
        {
            fn foo_at(
                __context__: &__Context__,
                _tag: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                <__Provider__ as DelegateComponent<
                    FooGetterAtComponent,
                >>::Delegate::foo_at(__context__, _tag)
            }
        }
        pub struct FooGetterAtComponent;
        impl<__Context__, I, J> FooGetterAt<__Context__, I, J> for UseContext
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: HasFooAt<I, J>,
        {
            fn foo_at(
                __context__: &__Context__,
                _tag: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                __Context__::foo_at(__context__, _tag)
            }
        }
        impl<__Context__, I, J> IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        for UseContext
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: HasFooAt<I, J>,
        {}
        impl<__Context__, I, J, __Components__, __Path__> FooGetterAt<__Context__, I, J>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >>::Delegate: FooGetterAt<__Context__, I, J>,
        {
            fn foo_at(
                __context__: &__Context__,
                _tag: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >>::Delegate::foo_at(__context__, _tag)
            }
        }
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Path__,
        > IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >>::Delegate: IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
                + FooGetterAt<__Context__, I, J>,
        {}
        impl<__Context__, I, J, __Components__, __Delegate__> FooGetterAt<__Context__, I, J>
        for UseDelegate<__Components__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Components__: DelegateComponent<(I), Delegate = __Delegate__>,
            __Delegate__: FooGetterAt<__Context__, I, J>,
        {
            fn foo_at(
                __context__: &__Context__,
                _tag: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                __Delegate__::foo_at(__context__, _tag)
            }
        }
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Delegate__,
        > IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        for UseDelegate<__Components__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Components__: DelegateComponent<(I), Delegate = __Delegate__>,
            __Delegate__: IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
                + FooGetterAt<__Context__, I, J>,
        {}
        impl<__Context__, I, J, __Components__, __Delegate__> FooGetterAt<__Context__, I, J>
        for UseDelegate2<__Components__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Components__: DelegateComponent<(I, J), Delegate = __Delegate__>,
            __Delegate__: FooGetterAt<__Context__, I, J>,
        {
            fn foo_at(
                __context__: &__Context__,
                _tag: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                __Delegate__::foo_at(__context__, _tag)
            }
        }
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Delegate__,
        > IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        for UseDelegate2<__Components__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Components__: DelegateComponent<(I, J), Delegate = __Delegate__>,
            __Delegate__: IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
                + FooGetterAt<__Context__, I, J>,
        {}
        impl<__Context__, I, J> FooGetterAt<__Context__, I, J> for UseFields
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'f', Chars<'o', Chars<'o', Chars<'_', Chars<'a', Chars<'t', Nil>>>>>>,
                >,
                Value = __Context__::Foo,
            >,
        {
            fn foo_at(
                __context__: &__Context__,
                _phantom: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'f',
                                    Chars<
                                        'o',
                                        Chars<'o', Chars<'_', Chars<'a', Chars<'t', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
            }
        }
        impl<__Context__, I, J> IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        for UseFields
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'f', Chars<'o', Chars<'o', Chars<'_', Chars<'a', Chars<'t', Nil>>>>>>,
                >,
                Value = __Context__::Foo,
            >,
        {}
        impl<__Context__, I, J, __Tag__> FooGetterAt<__Context__, I, J> for UseField<__Tag__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: HasField<__Tag__, Value = __Context__::Foo>,
        {
            fn foo_at(
                __context__: &__Context__,
                _phantom: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, I, J, __Tag__> IsProviderFor<FooGetterAtComponent, __Context__, (I, J)>
        for UseField<__Tag__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Context__: HasField<__Tag__, Value = __Context__::Foo>,
        {}
        impl<__Context__, I, J, __Provider__> FooGetterAt<__Context__, I, J>
        for WithProvider<__Provider__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Provider__: FieldGetter<
                __Context__,
                FooGetterAtComponent,
                Value = __Context__::Foo,
            >,
        {
            fn foo_at(
                __context__: &__Context__,
                _phantom: PhantomData<(I, J)>,
            ) -> &__Context__::Foo {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<FooGetterAtComponent>,
                )
            }
        }
        impl<
            __Context__,
            I,
            J,
            __Provider__,
        > IsProviderFor<FooGetterAtComponent, __Context__, (I, J)> for WithProvider<__Provider__>
        where
            __Context__: HasFooTypeAt<I, J>,
            __Provider__: FieldGetter<
                __Context__,
                FooGetterAtComponent,
                Value = __Context__::Foo,
            >,
        {}
        ")
    }
}

#[test]
pub fn test_derive_delegate() {
    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: String,
    }

    delegate_and_check_components! {
        MyContext {
            #[check_params(
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            )]
            FooTypeProviderAtComponent: UseDelegate<
                new FooTypes {
                    Index<1>: UseType<u64>,
                    Index<0>: UseType<String>,
                }
            >,

            #[check_params(
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            )]
            FooGetterAtComponent: UseDelegate<
                new FooGetters {
                    Index<1>: UseField<Symbol!("foo")>,
                    Index<0>: UseField<Symbol!("bar")>,
                }
            >
        }
    }

    check_components! {
        #[check_trait(CanUseMyContext)]
        MyContext {
            FooGetterAtComponent: [
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            ]
        }
    }

    let context = MyContext {
        foo: 42,
        bar: "Bar".into(),
    };

    assert_eq!(context.foo_at(PhantomData::<(Index<1>, Index<0>)>), &42);
    assert_eq!(context.foo_at(PhantomData::<(Index<0>, Index<1>)>), "Bar");
}

mod derive_delegate2 {
    use core::marker::PhantomData;

    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_delegate_components;
    use insta::assert_snapshot;

    use super::*;

    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: String,
    }

    snapshot_delegate_components! {
        delegate_components! {
            MyContext {
                FooTypeProviderAtComponent: UseDelegate2<
                    new FooTypes {
                        (Index<1>, Index<0>): UseType<u64>,
                        (Index<0>, Index<1>): UseType<String>,
                    }
                >,
                FooGetterAtComponent: UseDelegate2<
                    new FooGetters {
                        (Index<1>, Index<0>): UseField<Symbol!("foo")>,
                        (Index<0>, Index<1>): UseField<Symbol!("bar")>,
                    }
                >
            }
        }

        expand_my_context(output) {
            assert_snapshot!(output, @r#"
            pub struct FooTypes;
            pub struct FooGetters;
            impl DelegateComponent<FooTypeProviderAtComponent> for MyContext {
                type Delegate = UseDelegate2<FooTypes>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooTypeProviderAtComponent, __Context__, __Params__> for MyContext
            where
                UseDelegate2<
                    FooTypes,
                >: IsProviderFor<FooTypeProviderAtComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<FooGetterAtComponent> for MyContext {
                type Delegate = UseDelegate2<FooGetters>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooGetterAtComponent, __Context__, __Params__> for MyContext
            where
                UseDelegate2<
                    FooGetters,
                >: IsProviderFor<FooGetterAtComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<(Index<1>, Index<0>)> for FooTypes {
                type Delegate = UseType<u64>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<(Index<1>, Index<0>), __Context__, __Params__> for FooTypes
            where
                UseType<u64>: IsProviderFor<(Index<1>, Index<0>), __Context__, __Params__>,
            {}
            impl DelegateComponent<(Index<0>, Index<1>)> for FooTypes {
                type Delegate = UseType<String>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<(Index<0>, Index<1>), __Context__, __Params__> for FooTypes
            where
                UseType<String>: IsProviderFor<(Index<0>, Index<1>), __Context__, __Params__>,
            {}
            impl DelegateComponent<(Index<1>, Index<0>)> for FooGetters {
                type Delegate = UseField<Symbol!("foo")>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<(Index<1>, Index<0>), __Context__, __Params__> for FooGetters
            where
                UseField<
                    Symbol!("foo"),
                >: IsProviderFor<(Index<1>, Index<0>), __Context__, __Params__>,
            {}
            impl DelegateComponent<(Index<0>, Index<1>)> for FooGetters {
                type Delegate = UseField<Symbol!("bar")>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<(Index<0>, Index<1>), __Context__, __Params__> for FooGetters
            where
                UseField<
                    Symbol!("bar"),
                >: IsProviderFor<(Index<0>, Index<1>), __Context__, __Params__>,
            {}
            "#)
        }
    }

    check_components! {
        MyContext {
            FooGetterAtComponent: [
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            ]
        }
    }

    #[test]
    pub fn test_derive_delegate2() {
        let context = MyContext {
            foo: 42,
            bar: "Bar".into(),
        };

        assert_eq!(context.foo_at(PhantomData::<(Index<1>, Index<0>)>), &42);
        assert_eq!(context.foo_at(PhantomData::<(Index<0>, Index<1>)>), "Bar");
    }
}
