#![allow(dead_code)]

mod basic_check_components {
    use core::marker::PhantomData;

    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_getter, snapshot_cgp_type, snapshot_check_components,
        snapshot_delegate_and_check_components,
    };

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
        #[cgp_getter {
            provider: FooGetterAt,
        }]
        pub trait HasFooAt<I>: HasFooType {
            fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
        }

        expand_has_foo_at(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFooAt<I>: HasFooType {
                fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
            }
            impl<__Context__, I> HasFooAt<I> for __Context__
            where
                __Context__: HasFooType,
                __Context__: FooGetterAt<__Context__, I>,
            {
                fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo {
                    __Context__::foo(self, _tag)
                }
            }
            pub trait FooGetterAt<
                __Context__,
                I,
            >: IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            where
                __Context__: HasFooType,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo;
            }
            impl<__Provider__, __Context__, I> FooGetterAt<__Context__, I> for __Provider__
            where
                __Context__: HasFooType,
                __Provider__: DelegateComponent<FooGetterAtComponent>
                    + IsProviderFor<FooGetterAtComponent, __Context__, (I)>,
                <__Provider__ as DelegateComponent<
                    FooGetterAtComponent,
                >>::Delegate: FooGetterAt<__Context__, I>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                    <__Provider__ as DelegateComponent<
                        FooGetterAtComponent,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            pub struct FooGetterAtComponent;
            impl<__Context__, I> FooGetterAt<__Context__, I> for UseContext
            where
                __Context__: HasFooType,
                __Context__: HasFooAt<I>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                    __Context__::foo(__context__, _tag)
                }
            }
            impl<__Context__, I> IsProviderFor<FooGetterAtComponent, __Context__, (I)> for UseContext
            where
                __Context__: HasFooType,
                __Context__: HasFooAt<I>,
            {}
            impl<__Context__, I, __Components__, __Path__> FooGetterAt<__Context__, I>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasFooType,
                __Path__: ConcatPath<PathCons<I, Nil>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >>::Delegate: FooGetterAt<__Context__, I>,
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
            > IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasFooType,
                __Path__: ConcatPath<PathCons<I, Nil>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >>::Delegate: IsProviderFor<FooGetterAtComponent, __Context__, (I)>
                    + FooGetterAt<__Context__, I>,
            {}
            impl<__Context__, I> FooGetterAt<__Context__, I> for UseFields
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
            impl<__Context__, I> IsProviderFor<FooGetterAtComponent, __Context__, (I)> for UseFields
            where
                __Context__: HasFooType,
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = __Context__::Foo,
                >,
            {}
            impl<__Context__, I, __Tag__> FooGetterAt<__Context__, I> for UseField<__Tag__>
            where
                __Context__: HasFooType,
                __Context__: HasField<__Tag__, Value = __Context__::Foo>,
            {
                fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>)
                }
            }
            impl<__Context__, I, __Tag__> IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            for UseField<__Tag__>
            where
                __Context__: HasFooType,
                __Context__: HasField<__Tag__, Value = __Context__::Foo>,
            {}
            impl<__Context__, I, __Provider__> FooGetterAt<__Context__, I>
            for WithProvider<__Provider__>
            where
                __Context__: HasFooType,
                __Provider__: FieldGetter<
                    __Context__,
                    FooGetterAtComponent,
                    Value = __Context__::Foo,
                >,
            {
                fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                    __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<FooGetterAtComponent>,
                    )
                }
            }
            impl<__Context__, I, __Provider__> IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            for WithProvider<__Provider__>
            where
                __Context__: HasFooType,
                __Provider__: FieldGetter<
                    __Context__,
                    FooGetterAtComponent,
                    Value = __Context__::Foo,
                >,
            {}
            ")
        }
    }

    snapshot_cgp_getter! {
        #[cgp_getter {
            provider: BarGetterAt,
        }]
        pub trait HasBarAt<I, J>: HasBarType {
            fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
        }

        expand_has_bar_at(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasBarAt<I, J>: HasBarType {
                fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
            }
            impl<__Context__, I, J> HasBarAt<I, J> for __Context__
            where
                __Context__: HasBarType,
                __Context__: BarGetterAt<__Context__, I, J>,
            {
                fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar {
                    __Context__::foo(self, _tag)
                }
            }
            pub trait BarGetterAt<
                __Context__,
                I,
                J,
            >: IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>
            where
                __Context__: HasBarType,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar;
            }
            impl<__Provider__, __Context__, I, J> BarGetterAt<__Context__, I, J> for __Provider__
            where
                __Context__: HasBarType,
                __Provider__: DelegateComponent<BarGetterAtComponent>
                    + IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>,
                <__Provider__ as DelegateComponent<
                    BarGetterAtComponent,
                >>::Delegate: BarGetterAt<__Context__, I, J>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar {
                    <__Provider__ as DelegateComponent<
                        BarGetterAtComponent,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            pub struct BarGetterAtComponent;
            impl<__Context__, I, J> BarGetterAt<__Context__, I, J> for UseContext
            where
                __Context__: HasBarType,
                __Context__: HasBarAt<I, J>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar {
                    __Context__::foo(__context__, _tag)
                }
            }
            impl<__Context__, I, J> IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>
            for UseContext
            where
                __Context__: HasBarType,
                __Context__: HasBarAt<I, J>,
            {}
            impl<__Context__, I, J, __Components__, __Path__> BarGetterAt<__Context__, I, J>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasBarType,
                __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >>::Delegate: BarGetterAt<__Context__, I, J>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar {
                    <__Components__ as DelegateComponent<
                        <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            impl<
                __Context__,
                I,
                J,
                __Components__,
                __Path__,
            > IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasBarType,
                __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >>::Delegate: IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>
                    + BarGetterAt<__Context__, I, J>,
            {}
            impl<__Context__, I, J> BarGetterAt<__Context__, I, J> for UseFields
            where
                __Context__: HasBarType,
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = __Context__::Bar,
                >,
            {
                fn foo(
                    __context__: &__Context__,
                    _phantom: PhantomData<(I, J)>,
                ) -> &__Context__::Bar {
                    __context__
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                            >,
                        )
                }
            }
            impl<__Context__, I, J> IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>
            for UseFields
            where
                __Context__: HasBarType,
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = __Context__::Bar,
                >,
            {}
            impl<__Context__, I, J, __Tag__> BarGetterAt<__Context__, I, J> for UseField<__Tag__>
            where
                __Context__: HasBarType,
                __Context__: HasField<__Tag__, Value = __Context__::Bar>,
            {
                fn foo(
                    __context__: &__Context__,
                    _phantom: PhantomData<(I, J)>,
                ) -> &__Context__::Bar {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>)
                }
            }
            impl<__Context__, I, J, __Tag__> IsProviderFor<BarGetterAtComponent, __Context__, (I, J)>
            for UseField<__Tag__>
            where
                __Context__: HasBarType,
                __Context__: HasField<__Tag__, Value = __Context__::Bar>,
            {}
            impl<__Context__, I, J, __Provider__> BarGetterAt<__Context__, I, J>
            for WithProvider<__Provider__>
            where
                __Context__: HasBarType,
                __Provider__: FieldGetter<
                    __Context__,
                    BarGetterAtComponent,
                    Value = __Context__::Bar,
                >,
            {
                fn foo(
                    __context__: &__Context__,
                    _phantom: PhantomData<(I, J)>,
                ) -> &__Context__::Bar {
                    __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<BarGetterAtComponent>,
                    )
                }
            }
            impl<
                __Context__,
                I,
                J,
                __Provider__,
            > IsProviderFor<BarGetterAtComponent, __Context__, (I, J)> for WithProvider<__Provider__>
            where
                __Context__: HasBarType,
                __Provider__: FieldGetter<
                    __Context__,
                    BarGetterAtComponent,
                    Value = __Context__::Bar,
                >,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct Context {
        pub dummy: (),
        pub extra_dummy: (),
    }

    snapshot_delegate_and_check_components! {
        delegate_and_check_components! {
            Context {
                [
                    FooTypeProviderComponent,
                    BarTypeProviderComponent,
                ]:
                    UseType<()>,

                #[check_params(
                    (Index<5>, Index<6>),
                    (Index<7>, Index<8>),
                )]
                [
                    #[check_params(
                        Index<0>,
                        Index<1>,
                    )]
                    FooGetterAtComponent,

                    #[check_params(
                        (Index<0>, Index<1>),
                        (Index<1>, Index<0>),
                    )]
                    BarGetterAtComponent,
                ]:
                    UseField<Symbol!("dummy")>,
            }
        }

        expand_context(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<FooTypeProviderComponent> for Context {
                type Delegate = UseType<()>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooTypeProviderComponent, __Context__, __Params__> for Context
            where
                UseType<()>: IsProviderFor<FooTypeProviderComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<BarTypeProviderComponent> for Context {
                type Delegate = UseType<()>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<BarTypeProviderComponent, __Context__, __Params__> for Context
            where
                UseType<()>: IsProviderFor<BarTypeProviderComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<FooGetterAtComponent> for Context {
                type Delegate = UseField<Symbol!("dummy")>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooGetterAtComponent, __Context__, __Params__> for Context
            where
                UseField<
                    Symbol!("dummy"),
                >: IsProviderFor<FooGetterAtComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<BarGetterAtComponent> for Context {
                type Delegate = UseField<Symbol!("dummy")>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<BarGetterAtComponent, __Context__, __Params__> for Context
            where
                UseField<
                    Symbol!("dummy"),
                >: IsProviderFor<BarGetterAtComponent, __Context__, __Params__>,
            {}
            trait __CanUseContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl __CanUseContext<FooTypeProviderComponent, ()> for Context {}
            impl __CanUseContext<BarTypeProviderComponent, ()> for Context {}
            impl __CanUseContext<FooGetterAtComponent, (Index<5>, Index<6>)> for Context {}
            impl __CanUseContext<FooGetterAtComponent, (Index<7>, Index<8>)> for Context {}
            impl __CanUseContext<FooGetterAtComponent, Index<0>> for Context {}
            impl __CanUseContext<FooGetterAtComponent, Index<1>> for Context {}
            impl __CanUseContext<BarGetterAtComponent, (Index<5>, Index<6>)> for Context {}
            impl __CanUseContext<BarGetterAtComponent, (Index<7>, Index<8>)> for Context {}
            impl __CanUseContext<BarGetterAtComponent, (Index<0>, Index<1>)> for Context {}
            impl __CanUseContext<BarGetterAtComponent, (Index<1>, Index<0>)> for Context {}
            "#)
        }
    }

    snapshot_check_components! {
        check_components! {
            #[check_trait(CanUseContext)]
            Context {
                FooTypeProviderComponent,
                BarTypeProviderComponent,
                FooGetterAtComponent: [
                    Index<0>,
                    Index<1>,
                ],
                FooGetterAtComponent:
                    Index<3>,
            }

            #[check_trait(CanUseContext2)]
            Context {
                BarGetterAtComponent: [
                    (Index<0>, Index<1>),
                    (Index<1>, Index<0>),
                ],
                BarGetterAtComponent:
                    (Index<3>, Index<4>),
                [
                    FooGetterAtComponent,
                    BarGetterAtComponent,
                ]: [
                    (Index<5>, Index<6>),
                    (Index<7>, Index<8>),
                ]
            }

            #[check_trait(CanUseDummyField)]
            #[check_providers(
                UseField<Symbol!("dummy")>,
                UseField<Symbol!("extra_dummy")>,
            )]
            Context {
                FooGetterAtComponent: [
                    Index<0>,
                    Index<1>,
                ],
                FooGetterAtComponent:
                    Index<3>,
                BarGetterAtComponent: [
                    (Index<0>, Index<1>),
                    (Index<1>, Index<0>),
                ],
                BarGetterAtComponent:
                    (Index<3>, Index<4>),
                [
                    FooGetterAtComponent,
                    BarGetterAtComponent,
                ]: [
                    (Index<5>, Index<6>),
                    (Index<7>, Index<8>),
                ]
            }
        }

        expand_check_context(output) {
            insta::assert_snapshot!(output, @r#"
            trait CanUseContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl CanUseContext<FooTypeProviderComponent, ()> for Context {}
            impl CanUseContext<BarTypeProviderComponent, ()> for Context {}
            impl CanUseContext<FooGetterAtComponent, Index<0>> for Context {}
            impl CanUseContext<FooGetterAtComponent, Index<1>> for Context {}
            impl CanUseContext<FooGetterAtComponent, Index<3>> for Context {}
            trait CanUseContext2<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl CanUseContext2<BarGetterAtComponent, (Index<0>, Index<1>)> for Context {}
            impl CanUseContext2<BarGetterAtComponent, (Index<1>, Index<0>)> for Context {}
            impl CanUseContext2<BarGetterAtComponent, (Index<3>, Index<4>)> for Context {}
            impl CanUseContext2<FooGetterAtComponent, (Index<5>, Index<6>)> for Context {}
            impl CanUseContext2<FooGetterAtComponent, (Index<7>, Index<8>)> for Context {}
            impl CanUseContext2<BarGetterAtComponent, (Index<5>, Index<6>)> for Context {}
            impl CanUseContext2<BarGetterAtComponent, (Index<7>, Index<8>)> for Context {}
            trait CanUseDummyField<
                __Component__,
                __Params__: ?Sized,
            >: IsProviderFor<__Component__, Context, __Params__> {}
            impl CanUseDummyField<FooGetterAtComponent, Index<0>> for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, Index<0>>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, Index<1>> for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, Index<1>>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, Index<3>> for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, Index<3>>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<0>, Index<1>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<0>, Index<1>)>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<1>, Index<0>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<1>, Index<0>)>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<3>, Index<4>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<3>, Index<4>)>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, (Index<5>, Index<6>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, (Index<5>, Index<6>)>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, (Index<7>, Index<8>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<FooGetterAtComponent, (Index<7>, Index<8>)>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<5>, Index<6>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<5>, Index<6>)>
            for UseField<Symbol!("extra_dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<7>, Index<8>)>
            for UseField<Symbol!("dummy")> {}
            impl CanUseDummyField<BarGetterAtComponent, (Index<7>, Index<8>)>
            for UseField<Symbol!("extra_dummy")> {}
            "#)
        }
    }
}

mod generic_check_components {
    use core::marker::PhantomData;

    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_getter, snapshot_cgp_type, snapshot_check_components,
        snapshot_delegate_components,
    };

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
        #[cgp_getter {
            provider: FooGetterAt,
        }]
        pub trait HasFooAt<I: Clone>: HasFooType {
            fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
        }

        expand_has_foo_at(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasFooAt<I: Clone>: HasFooType {
                fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
            }
            impl<__Context__, I: Clone> HasFooAt<I> for __Context__
            where
                __Context__: HasFooType,
                __Context__: FooGetterAt<__Context__, I>,
            {
                fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo {
                    __Context__::foo(self, _tag)
                }
            }
            pub trait FooGetterAt<
                __Context__,
                I: Clone,
            >: IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            where
                __Context__: HasFooType,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo;
            }
            impl<__Provider__, __Context__, I: Clone> FooGetterAt<__Context__, I> for __Provider__
            where
                __Context__: HasFooType,
                __Provider__: DelegateComponent<FooGetterAtComponent>
                    + IsProviderFor<FooGetterAtComponent, __Context__, (I)>,
                <__Provider__ as DelegateComponent<
                    FooGetterAtComponent,
                >>::Delegate: FooGetterAt<__Context__, I>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                    <__Provider__ as DelegateComponent<
                        FooGetterAtComponent,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            pub struct FooGetterAtComponent;
            impl<__Context__, I: Clone> FooGetterAt<__Context__, I> for UseContext
            where
                __Context__: HasFooType,
                __Context__: HasFooAt<I>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                    __Context__::foo(__context__, _tag)
                }
            }
            impl<__Context__, I: Clone> IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            for UseContext
            where
                __Context__: HasFooType,
                __Context__: HasFooAt<I>,
            {}
            impl<__Context__, I: Clone, __Components__, __Path__> FooGetterAt<__Context__, I>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasFooType,
                __Path__: ConcatPath<PathCons<I, Nil>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >>::Delegate: FooGetterAt<__Context__, I>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<I>) -> &__Context__::Foo {
                    <__Components__ as DelegateComponent<
                        <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            impl<
                __Context__,
                I: Clone,
                __Components__,
                __Path__,
            > IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasFooType,
                __Path__: ConcatPath<PathCons<I, Nil>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, Nil>>>::Output,
                >>::Delegate: IsProviderFor<FooGetterAtComponent, __Context__, (I)>
                    + FooGetterAt<__Context__, I>,
            {}
            impl<__Context__, I: Clone> FooGetterAt<__Context__, I> for UseFields
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
            impl<__Context__, I: Clone> IsProviderFor<FooGetterAtComponent, __Context__, (I)>
            for UseFields
            where
                __Context__: HasFooType,
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = __Context__::Foo,
                >,
            {}
            impl<__Context__, I: Clone, __Tag__> FooGetterAt<__Context__, I> for UseField<__Tag__>
            where
                __Context__: HasFooType,
                __Context__: HasField<__Tag__, Value = __Context__::Foo>,
            {
                fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>)
                }
            }
            impl<
                __Context__,
                I: Clone,
                __Tag__,
            > IsProviderFor<FooGetterAtComponent, __Context__, (I)> for UseField<__Tag__>
            where
                __Context__: HasFooType,
                __Context__: HasField<__Tag__, Value = __Context__::Foo>,
            {}
            impl<__Context__, I: Clone, __Provider__> FooGetterAt<__Context__, I>
            for WithProvider<__Provider__>
            where
                __Context__: HasFooType,
                __Provider__: FieldGetter<
                    __Context__,
                    FooGetterAtComponent,
                    Value = __Context__::Foo,
                >,
            {
                fn foo(__context__: &__Context__, _phantom: PhantomData<I>) -> &__Context__::Foo {
                    __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<FooGetterAtComponent>,
                    )
                }
            }
            impl<
                __Context__,
                I: Clone,
                __Provider__,
            > IsProviderFor<FooGetterAtComponent, __Context__, (I)> for WithProvider<__Provider__>
            where
                __Context__: HasFooType,
                __Provider__: FieldGetter<
                    __Context__,
                    FooGetterAtComponent,
                    Value = __Context__::Foo,
                >,
            {}
            ")
        }
    }

    snapshot_cgp_getter! {
        #[cgp_getter {
            name: BarGetterAtComponent<I>,
            provider: BarGetterAt,
        }]
        pub trait HasBarAt<I: Clone, J>: HasBarType {
            fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
        }

        expand_has_bar_at(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasBarAt<I: Clone, J>: HasBarType {
                fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
            }
            impl<__Context__, I: Clone, J> HasBarAt<I, J> for __Context__
            where
                __Context__: HasBarType,
                __Context__: BarGetterAt<__Context__, I, J>,
            {
                fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar {
                    __Context__::foo(self, _tag)
                }
            }
            pub trait BarGetterAt<
                __Context__,
                I: Clone,
                J,
            >: IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)>
            where
                __Context__: HasBarType,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar;
            }
            impl<__Provider__, __Context__, I: Clone, J> BarGetterAt<__Context__, I, J>
            for __Provider__
            where
                __Context__: HasBarType,
                __Provider__: DelegateComponent<BarGetterAtComponent<I>>
                    + IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)>,
                <__Provider__ as DelegateComponent<
                    BarGetterAtComponent<I>,
                >>::Delegate: BarGetterAt<__Context__, I, J>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar {
                    <__Provider__ as DelegateComponent<
                        BarGetterAtComponent<I>,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            pub struct BarGetterAtComponent<I>(pub ::core::marker::PhantomData<(I)>);
            impl<__Context__, I: Clone, J> BarGetterAt<__Context__, I, J> for UseContext
            where
                __Context__: HasBarType,
                __Context__: HasBarAt<I, J>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar {
                    __Context__::foo(__context__, _tag)
                }
            }
            impl<
                __Context__,
                I: Clone,
                J,
            > IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)> for UseContext
            where
                __Context__: HasBarType,
                __Context__: HasBarAt<I, J>,
            {}
            impl<__Context__, I: Clone, J, __Components__, __Path__> BarGetterAt<__Context__, I, J>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasBarType,
                __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >>::Delegate: BarGetterAt<__Context__, I, J>,
            {
                fn foo(__context__: &__Context__, _tag: PhantomData<(I, J)>) -> &__Context__::Bar {
                    <__Components__ as DelegateComponent<
                        <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                    >>::Delegate::foo(__context__, _tag)
                }
            }
            impl<
                __Context__,
                I: Clone,
                J,
                __Components__,
                __Path__,
            > IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasBarType,
                __Path__: ConcatPath<PathCons<I, PathCons<J, Nil>>>,
                __Components__: DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >,
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<I, PathCons<J, Nil>>>>::Output,
                >>::Delegate: IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)>
                    + BarGetterAt<__Context__, I, J>,
            {}
            impl<__Context__, I: Clone, J> BarGetterAt<__Context__, I, J> for UseFields
            where
                __Context__: HasBarType,
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = __Context__::Bar,
                >,
            {
                fn foo(
                    __context__: &__Context__,
                    _phantom: PhantomData<(I, J)>,
                ) -> &__Context__::Bar {
                    __context__
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                            >,
                        )
                }
            }
            impl<
                __Context__,
                I: Clone,
                J,
            > IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)> for UseFields
            where
                __Context__: HasBarType,
                __Context__: HasField<
                    Symbol<3, Chars<'f', Chars<'o', Chars<'o', Nil>>>>,
                    Value = __Context__::Bar,
                >,
            {}
            impl<__Context__, I: Clone, J, __Tag__> BarGetterAt<__Context__, I, J>
            for UseField<__Tag__>
            where
                __Context__: HasBarType,
                __Context__: HasField<__Tag__, Value = __Context__::Bar>,
            {
                fn foo(
                    __context__: &__Context__,
                    _phantom: PhantomData<(I, J)>,
                ) -> &__Context__::Bar {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>)
                }
            }
            impl<
                __Context__,
                I: Clone,
                J,
                __Tag__,
            > IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)> for UseField<__Tag__>
            where
                __Context__: HasBarType,
                __Context__: HasField<__Tag__, Value = __Context__::Bar>,
            {}
            impl<__Context__, I: Clone, J, __Provider__> BarGetterAt<__Context__, I, J>
            for WithProvider<__Provider__>
            where
                __Context__: HasBarType,
                __Provider__: FieldGetter<
                    __Context__,
                    BarGetterAtComponent<I>,
                    Value = __Context__::Bar,
                >,
            {
                fn foo(
                    __context__: &__Context__,
                    _phantom: PhantomData<(I, J)>,
                ) -> &__Context__::Bar {
                    __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<BarGetterAtComponent<I>>,
                    )
                }
            }
            impl<
                __Context__,
                I: Clone,
                J,
                __Provider__,
            > IsProviderFor<BarGetterAtComponent<I>, __Context__, (I, J)>
            for WithProvider<__Provider__>
            where
                __Context__: HasBarType,
                __Provider__: FieldGetter<
                    __Context__,
                    BarGetterAtComponent<I>,
                    Value = __Context__::Bar,
                >,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct Context {
        pub dummy: (),
    }

    snapshot_delegate_components! {
        delegate_components! {
            Context {
                [
                    FooTypeProviderComponent,
                    BarTypeProviderComponent,
                ]:
                    UseType<()>,
                [
                    FooGetterAtComponent,
                    <I> BarGetterAtComponent<I>,
                ]:
                    UseField<Symbol!("dummy")>,
            }
        }

        expand_context(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<FooTypeProviderComponent> for Context {
                type Delegate = UseType<()>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooTypeProviderComponent, __Context__, __Params__> for Context
            where
                UseType<()>: IsProviderFor<FooTypeProviderComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<BarTypeProviderComponent> for Context {
                type Delegate = UseType<()>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<BarTypeProviderComponent, __Context__, __Params__> for Context
            where
                UseType<()>: IsProviderFor<BarTypeProviderComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<FooGetterAtComponent> for Context {
                type Delegate = UseField<Symbol!("dummy")>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<FooGetterAtComponent, __Context__, __Params__> for Context
            where
                UseField<
                    Symbol!("dummy"),
                >: IsProviderFor<FooGetterAtComponent, __Context__, __Params__>,
            {}
            impl<I> DelegateComponent<BarGetterAtComponent<I>> for Context {
                type Delegate = UseField<Symbol!("dummy")>;
            }
            impl<
                I,
                __Context__,
                __Params__,
            > IsProviderFor<BarGetterAtComponent<I>, __Context__, __Params__> for Context
            where
                UseField<
                    Symbol!("dummy"),
                >: IsProviderFor<BarGetterAtComponent<I>, __Context__, __Params__>,
            {}
            "#)
        }
    }

    snapshot_check_components! {
        check_components! {
            <'a, I> Context
            where
                I: Clone,
            {
                FooGetterAtComponent: &'a I,
                BarGetterAtComponent<I>: (I, &'a Index<0>),
            }
        }

        expand_check_context(output) {
            insta::assert_snapshot!(output, @"
            trait __CheckContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl<'a, I> __CheckContext<FooGetterAtComponent, &'a I> for Context
            where
                I: Clone,
            {}
            impl<'a, I> __CheckContext<BarGetterAtComponent<I>, (I, &'a Index<0>)> for Context
            where
                I: Clone,
            {}
            ")
        }
    }
}
