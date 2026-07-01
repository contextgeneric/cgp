//! The `UseDelegate` dispatch provider and the `UseDelegate`-table form of
//! `delegate_components!` / `delegate_and_check_components!` — this concept owns
//! those snapshots.
//!
//! A multi-parameter getter component (`HasFooAt<I, J>`) is dispatched per
//! `(I, J)` value by wiring the component to a `UseDelegate` (single-key `I`) or
//! a custom `UseDelegate2` (tuple key `(I, J)`) table. `#[derive_delegate(...)]`
//! generates the dispatch-provider impls for each table type; the retained
//! `snapshot_cgp_type!` / `snapshot_cgp_getter!` expansions pin those `UseDelegate`
//! / `UseDelegate2` impls, and the retained `snapshot_delegate_*` expansions pin
//! the `UseDelegate`-table wiring. The `check_components!` scaffolding uses the
//! plain macro (checking is owned by another target).
//!
//! See docs/reference/providers/use_delegate.md and
//! docs/reference/providers/dispatch_combinators.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_cgp_type};

pub struct UseDelegate2<Components>(pub PhantomData<Components>);

snapshot_cgp_type! {
    #[cgp_type(FooTypeProviderAt)]
    #[derive_delegate(UseDelegate<I>)]
    #[derive_delegate(UseDelegate2<(I, J)>)]
    pub trait HasFooTypeAt<I, J> {
        type Foo;
    }

    expand_has_foo_type_at(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooTypeAt<I, J> {
            type Foo;
        }
        impl<__Context__, I, J> HasFooTypeAt<I, J> for __Context__
        where
            __Context__: FooTypeProviderAt<__Context__, I, J>,
        {
            type Foo = <__Context__ as FooTypeProviderAt<__Context__, I, J>>::Foo;
        }
        pub trait FooTypeProviderAt<
            __Context__,
            I,
            J,
        >: IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)> {
            type Foo;
        }
        impl<__Provider__, __Context__, I, J> FooTypeProviderAt<__Context__, I, J>
        for __Provider__
        where
            __Provider__: DelegateComponent<FooTypeProviderAtComponent>
                + IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>,
            <__Provider__ as DelegateComponent<
                FooTypeProviderAtComponent,
            >>::Delegate: FooTypeProviderAt<__Context__, I, J>,
        {
            type Foo = <<__Provider__ as DelegateComponent<
                FooTypeProviderAtComponent,
            >>::Delegate as FooTypeProviderAt<__Context__, I, J>>::Foo;
        }
        pub struct FooTypeProviderAtComponent;
        impl<__Context__, I, J> FooTypeProviderAt<__Context__, I, J> for UseContext
        where
            __Context__: HasFooTypeAt<I, J>,
        {
            type Foo = <__Context__ as HasFooTypeAt<I, J>>::Foo;
        }
        impl<__Context__, I, J> IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
        for UseContext
        where
            __Context__: HasFooTypeAt<I, J>,
        {}
        impl<__Context__, I, J, __Components__, __Path__> FooTypeProviderAt<__Context__, I, J>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >>::Delegate: FooTypeProviderAt<__Context__, I, J>,
        {
            type Foo = <<__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >>::Delegate as FooTypeProviderAt<__Context__, I, J>>::Foo;
        }
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Path__,
        > IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
            >>::Delegate: IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
                + FooTypeProviderAt<__Context__, I, J>,
        {}
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Delegate__,
        > FooTypeProviderAt<__Context__, I, J> for UseDelegate<__Components__>
        where
            __Components__: DelegateComponent<(I), Delegate = __Delegate__>,
            __Delegate__: FooTypeProviderAt<__Context__, I, J>,
        {
            type Foo = <__Delegate__ as FooTypeProviderAt<__Context__, I, J>>::Foo;
        }
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Delegate__,
        > IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
        for UseDelegate<__Components__>
        where
            __Components__: DelegateComponent<(I), Delegate = __Delegate__>,
            __Delegate__: IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
                + FooTypeProviderAt<__Context__, I, J>,
        {}
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Delegate__,
        > FooTypeProviderAt<__Context__, I, J> for UseDelegate2<__Components__>
        where
            __Components__: DelegateComponent<(I, J), Delegate = __Delegate__>,
            __Delegate__: FooTypeProviderAt<__Context__, I, J>,
        {
            type Foo = <__Delegate__ as FooTypeProviderAt<__Context__, I, J>>::Foo;
        }
        impl<
            __Context__,
            I,
            J,
            __Components__,
            __Delegate__,
        > IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
        for UseDelegate2<__Components__>
        where
            __Components__: DelegateComponent<(I, J), Delegate = __Delegate__>,
            __Delegate__: IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
                + FooTypeProviderAt<__Context__, I, J>,
        {}
        impl<Foo, __Context__, I, J> FooTypeProviderAt<__Context__, I, J> for UseType<Foo> {
            type Foo = Foo;
        }
        impl<
            Foo,
            __Context__,
            I,
            J,
        > IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)> for UseType<Foo> {}
        impl<__Provider__, Foo, __Context__, I, J> FooTypeProviderAt<__Context__, I, J>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, FooTypeProviderAtComponent, Type = Foo>,
        {
            type Foo = Foo;
        }
        impl<
            __Provider__,
            Foo,
            __Context__,
            I,
            J,
        > IsProviderFor<FooTypeProviderAtComponent, __Context__, (I, J)>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, FooTypeProviderAtComponent, Type = Foo>,
        {}
        ")
    }
}

snapshot_cgp_getter! {
    #[cgp_getter(FooGetterAt)]
    #[derive_delegate(UseDelegate<I>)]
    #[derive_delegate(UseDelegate2<(I, J)>)]
    pub trait HasFooAt<I, J>: HasFooTypeAt<I, J> {
        fn foo_at(&self, _tag: PhantomData<(I, J)>) -> &Self::Foo;
    }

    expand_has_foo_at(output) {
        insta::assert_snapshot!(output, @"
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

mod derive_delegate {
    use core::marker::PhantomData;

    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_delegate_and_check_components;

    use super::*;

    #[derive(HasField)]
    pub struct MyContext {
        pub foo: u64,
        pub bar: String,
    }

    snapshot_delegate_and_check_components! {
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

        expand_my_context(output) {
            insta::assert_snapshot!(output, @r#"
            pub struct FooTypes;
            pub struct FooGetters;
            impl DelegateComponent<FooTypeProviderAtComponent> for MyContext {
                type Delegate = UseDelegate<FooTypes>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooTypeProviderAtComponent, __Context__, __Params__> for MyContext
            where
                UseDelegate<
                    FooTypes,
                >: IsProviderFor<FooTypeProviderAtComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<FooGetterAtComponent> for MyContext {
                type Delegate = UseDelegate<FooGetters>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooGetterAtComponent, __Context__, __Params__> for MyContext
            where
                UseDelegate<
                    FooGetters,
                >: IsProviderFor<FooGetterAtComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<Index<1>> for FooTypes {
                type Delegate = UseType<u64>;
            }
            impl<__Context__, __Params__> IsProviderFor<Index<1>, __Context__, __Params__>
            for FooTypes
            where
                UseType<u64>: IsProviderFor<Index<1>, __Context__, __Params__>,
            {}
            impl DelegateComponent<Index<0>> for FooTypes {
                type Delegate = UseType<String>;
            }
            impl<__Context__, __Params__> IsProviderFor<Index<0>, __Context__, __Params__>
            for FooTypes
            where
                UseType<String>: IsProviderFor<Index<0>, __Context__, __Params__>,
            {}
            impl DelegateComponent<Index<1>> for FooGetters {
                type Delegate = UseField<Symbol!("foo")>;
            }
            impl<__Context__, __Params__> IsProviderFor<Index<1>, __Context__, __Params__>
            for FooGetters
            where
                UseField<Symbol!("foo")>: IsProviderFor<Index<1>, __Context__, __Params__>,
            {}
            impl DelegateComponent<Index<0>> for FooGetters {
                type Delegate = UseField<Symbol!("bar")>;
            }
            impl<__Context__, __Params__> IsProviderFor<Index<0>, __Context__, __Params__>
            for FooGetters
            where
                UseField<Symbol!("bar")>: IsProviderFor<Index<0>, __Context__, __Params__>,
            {}
            trait __CanUseMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl __CanUseMyContext<FooTypeProviderAtComponent, (Index<1>, Index<0>)> for MyContext {}
            impl __CanUseMyContext<FooTypeProviderAtComponent, (Index<0>, Index<1>)> for MyContext {}
            impl __CanUseMyContext<FooGetterAtComponent, (Index<1>, Index<0>)> for MyContext {}
            impl __CanUseMyContext<FooGetterAtComponent, (Index<0>, Index<1>)> for MyContext {}
            "#)
        }
    }

    // Checking is owned by another concept target, so this uses the plain macro.
    check_components! {
        #[check_trait(CanUseMyContext)]
        MyContext {
            FooGetterAtComponent: [
                (Index<1>, Index<0>),
                (Index<0>, Index<1>),
            ]
        }
    }

    #[test]
    pub fn test_derive_delegate() {
        let context = MyContext {
            foo: 42,
            bar: "Bar".into(),
        };

        assert_eq!(context.foo_at(PhantomData::<(Index<1>, Index<0>)>), &42);
        assert_eq!(context.foo_at(PhantomData::<(Index<0>, Index<1>)>), "Bar");
    }
}

mod derive_delegate2 {
    use core::marker::PhantomData;

    use cgp::prelude::*;
    use cgp_macro_test_util::snapshot_delegate_components;

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
            insta::assert_snapshot!(output, @r#"
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

    // Checking is owned by another concept target, so this uses the plain macro.
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
