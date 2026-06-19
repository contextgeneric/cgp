#![allow(dead_code)]

mod basic_delegate_and_check_components {
    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_getter, snapshot_cgp_type, snapshot_delegate_and_check_components,
    };

    snapshot_cgp_type! {
        #[cgp_type]
        pub trait HasNameType {
            type Name;
        }

        expand_has_name_type(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasNameType {
                type Name;
            }
            impl<__Context__> HasNameType for __Context__
            where
                __Context__: NameTypeProvider<__Context__>,
            {
                type Name = <__Context__ as NameTypeProvider<__Context__>>::Name;
            }
            pub trait NameTypeProvider<
                __Context__,
            >: IsProviderFor<NameTypeProviderComponent, __Context__, ()> {
                type Name;
            }
            impl<__Provider__, __Context__> NameTypeProvider<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<NameTypeProviderComponent>
                    + IsProviderFor<NameTypeProviderComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    NameTypeProviderComponent,
                >>::Delegate: NameTypeProvider<__Context__>,
            {
                type Name = <<__Provider__ as DelegateComponent<
                    NameTypeProviderComponent,
                >>::Delegate as NameTypeProvider<__Context__>>::Name;
            }
            pub struct NameTypeProviderComponent;
            impl<__Context__> NameTypeProvider<__Context__> for UseContext
            where
                __Context__: HasNameType,
            {
                type Name = <__Context__ as HasNameType>::Name;
            }
            impl<__Context__> IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for UseContext
            where
                __Context__: HasNameType,
            {}
            impl<__Context__, __Components__, __Path__> NameTypeProvider<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: NameTypeProvider<__Context__>,
            {
                type Name = <<__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate as NameTypeProvider<__Context__>>::Name;
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<NameTypeProviderComponent, __Context__, ()>
                    + NameTypeProvider<__Context__>,
            {}
            impl<Name, __Context__> NameTypeProvider<__Context__> for UseType<Name>
            where
                Name:,
            {
                type Name = Name;
            }
            impl<Name, __Context__> IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for UseType<Name>
            where
                Name:,
            {}
            impl<__Provider__, Name, __Context__> NameTypeProvider<__Context__>
            for WithProvider<__Provider__>
            where
                __Provider__: TypeProvider<__Context__, NameTypeProviderComponent, Type = Name>,
                Name:,
            {
                type Name = Name;
            }
            impl<
                __Provider__,
                Name,
                __Context__,
            > IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Provider__: TypeProvider<__Context__, NameTypeProviderComponent, Type = Name>,
                Name:,
            {}
            ")
        }
    }

    snapshot_cgp_getter! {
        #[cgp_getter]
        pub trait HasName: HasNameType {
            fn name(&self) -> &Self::Name;
        }

        expand_has_name(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasName: HasNameType {
                fn name(&self) -> &Self::Name;
            }
            impl<__Context__> HasName for __Context__
            where
                __Context__: HasNameType,
                __Context__: NameGetter<__Context__>,
            {
                fn name(&self) -> &Self::Name {
                    __Context__::name(self)
                }
            }
            pub trait NameGetter<__Context__>: IsProviderFor<NameGetterComponent, __Context__, ()>
            where
                __Context__: HasNameType,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name;
            }
            impl<__Provider__, __Context__> NameGetter<__Context__> for __Provider__
            where
                __Context__: HasNameType,
                __Provider__: DelegateComponent<NameGetterComponent>
                    + IsProviderFor<NameGetterComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    NameGetterComponent,
                >>::Delegate: NameGetter<__Context__>,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    <__Provider__ as DelegateComponent<
                        NameGetterComponent,
                    >>::Delegate::name(__context__)
                }
            }
            pub struct NameGetterComponent;
            impl<__Context__> NameGetter<__Context__> for UseContext
            where
                __Context__: HasNameType,
                __Context__: HasName,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __Context__::name(__context__)
                }
            }
            impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseContext
            where
                __Context__: HasNameType,
                __Context__: HasName,
            {}
            impl<__Context__, __Components__, __Path__> NameGetter<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasNameType,
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<__Path__>>::Delegate: NameGetter<__Context__>,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    <__Components__ as DelegateComponent<__Path__>>::Delegate::name(__context__)
                }
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<NameGetterComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasNameType,
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<NameGetterComponent, __Context__, ()>
                    + NameGetter<__Context__>,
            {}
            impl<__Context__> NameGetter<__Context__> for UseFields
            where
                __Context__: HasNameType,
                __Context__: HasField<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    Value = __Context__::Name,
                >,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __context__
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        )
                }
            }
            impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseFields
            where
                __Context__: HasNameType,
                __Context__: HasField<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    Value = __Context__::Name,
                >,
            {}
            impl<__Context__, __Tag__> NameGetter<__Context__> for UseField<__Tag__>
            where
                __Context__: HasNameType,
                __Context__: HasField<__Tag__, Value = __Context__::Name>,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>)
                }
            }
            impl<__Context__, __Tag__> IsProviderFor<NameGetterComponent, __Context__, ()>
            for UseField<__Tag__>
            where
                __Context__: HasNameType,
                __Context__: HasField<__Tag__, Value = __Context__::Name>,
            {}
            impl<__Context__, __Provider__> NameGetter<__Context__> for WithProvider<__Provider__>
            where
                __Context__: HasNameType,
                __Provider__: FieldGetter<
                    __Context__,
                    NameGetterComponent,
                    Value = __Context__::Name,
                >,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<NameGetterComponent>,
                    )
                }
            }
            impl<__Context__, __Provider__> IsProviderFor<NameGetterComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Context__: HasNameType,
                __Provider__: FieldGetter<
                    __Context__,
                    NameGetterComponent,
                    Value = __Context__::Name,
                >,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct MyContext {
        pub name: String,
    }

    snapshot_delegate_and_check_components! {
        delegate_and_check_components! {
            #[check_trait(CheckMyContext)]
            MyContext {
                NameTypeProviderComponent: UseType<String>,
                NameGetterComponent: UseField<Symbol!("name")>,
            }
        }

        expand_my_context(output) {
            insta::assert_snapshot!(output, @r#"
            impl DelegateComponent<NameTypeProviderComponent> for MyContext {
                type Delegate = UseType<String>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for MyContext
            where
                UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
            {}
            impl DelegateComponent<NameGetterComponent> for MyContext {
                type Delegate = UseField<Symbol!("name")>;
            }
            impl<__Context__, __Params__> IsProviderFor<NameGetterComponent, __Context__, __Params__>
            for MyContext
            where
                UseField<
                    Symbol!("name"),
                >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
            {}
            trait CheckMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl CheckMyContext<NameTypeProviderComponent, ()> for MyContext {}
            impl CheckMyContext<NameGetterComponent, ()> for MyContext {}
            "#)
        }
    }
}

mod generic_delegate_and_check_components {
    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_getter, snapshot_cgp_type, snapshot_delegate_and_check_components,
    };

    snapshot_cgp_type! {
        #[cgp_type]
        pub trait HasNameType {
            type Name;
        }

        expand_has_name_type(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasNameType {
                type Name;
            }
            impl<__Context__> HasNameType for __Context__
            where
                __Context__: NameTypeProvider<__Context__>,
            {
                type Name = <__Context__ as NameTypeProvider<__Context__>>::Name;
            }
            pub trait NameTypeProvider<
                __Context__,
            >: IsProviderFor<NameTypeProviderComponent, __Context__, ()> {
                type Name;
            }
            impl<__Provider__, __Context__> NameTypeProvider<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<NameTypeProviderComponent>
                    + IsProviderFor<NameTypeProviderComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    NameTypeProviderComponent,
                >>::Delegate: NameTypeProvider<__Context__>,
            {
                type Name = <<__Provider__ as DelegateComponent<
                    NameTypeProviderComponent,
                >>::Delegate as NameTypeProvider<__Context__>>::Name;
            }
            pub struct NameTypeProviderComponent;
            impl<__Context__> NameTypeProvider<__Context__> for UseContext
            where
                __Context__: HasNameType,
            {
                type Name = <__Context__ as HasNameType>::Name;
            }
            impl<__Context__> IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for UseContext
            where
                __Context__: HasNameType,
            {}
            impl<__Context__, __Components__, __Path__> NameTypeProvider<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: NameTypeProvider<__Context__>,
            {
                type Name = <<__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate as NameTypeProvider<__Context__>>::Name;
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<NameTypeProviderComponent, __Context__, ()>
                    + NameTypeProvider<__Context__>,
            {}
            impl<Name, __Context__> NameTypeProvider<__Context__> for UseType<Name>
            where
                Name:,
            {
                type Name = Name;
            }
            impl<Name, __Context__> IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for UseType<Name>
            where
                Name:,
            {}
            impl<__Provider__, Name, __Context__> NameTypeProvider<__Context__>
            for WithProvider<__Provider__>
            where
                __Provider__: TypeProvider<__Context__, NameTypeProviderComponent, Type = Name>,
                Name:,
            {
                type Name = Name;
            }
            impl<
                __Provider__,
                Name,
                __Context__,
            > IsProviderFor<NameTypeProviderComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Provider__: TypeProvider<__Context__, NameTypeProviderComponent, Type = Name>,
                Name:,
            {}
            ")
        }
    }

    snapshot_cgp_getter! {
        #[cgp_getter]
        pub trait HasName: HasNameType {
            fn name(&self) -> &Self::Name;
        }

        expand_has_name(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasName: HasNameType {
                fn name(&self) -> &Self::Name;
            }
            impl<__Context__> HasName for __Context__
            where
                __Context__: HasNameType,
                __Context__: NameGetter<__Context__>,
            {
                fn name(&self) -> &Self::Name {
                    __Context__::name(self)
                }
            }
            pub trait NameGetter<__Context__>: IsProviderFor<NameGetterComponent, __Context__, ()>
            where
                __Context__: HasNameType,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name;
            }
            impl<__Provider__, __Context__> NameGetter<__Context__> for __Provider__
            where
                __Context__: HasNameType,
                __Provider__: DelegateComponent<NameGetterComponent>
                    + IsProviderFor<NameGetterComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    NameGetterComponent,
                >>::Delegate: NameGetter<__Context__>,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    <__Provider__ as DelegateComponent<
                        NameGetterComponent,
                    >>::Delegate::name(__context__)
                }
            }
            pub struct NameGetterComponent;
            impl<__Context__> NameGetter<__Context__> for UseContext
            where
                __Context__: HasNameType,
                __Context__: HasName,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __Context__::name(__context__)
                }
            }
            impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseContext
            where
                __Context__: HasNameType,
                __Context__: HasName,
            {}
            impl<__Context__, __Components__, __Path__> NameGetter<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasNameType,
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<__Path__>>::Delegate: NameGetter<__Context__>,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    <__Components__ as DelegateComponent<__Path__>>::Delegate::name(__context__)
                }
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<NameGetterComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Context__: HasNameType,
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<NameGetterComponent, __Context__, ()>
                    + NameGetter<__Context__>,
            {}
            impl<__Context__> NameGetter<__Context__> for UseFields
            where
                __Context__: HasNameType,
                __Context__: HasField<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    Value = __Context__::Name,
                >,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __context__
                        .get_field(
                            ::core::marker::PhantomData::<
                                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        )
                }
            }
            impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseFields
            where
                __Context__: HasNameType,
                __Context__: HasField<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    Value = __Context__::Name,
                >,
            {}
            impl<__Context__, __Tag__> NameGetter<__Context__> for UseField<__Tag__>
            where
                __Context__: HasNameType,
                __Context__: HasField<__Tag__, Value = __Context__::Name>,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __context__.get_field(::core::marker::PhantomData::<__Tag__>)
                }
            }
            impl<__Context__, __Tag__> IsProviderFor<NameGetterComponent, __Context__, ()>
            for UseField<__Tag__>
            where
                __Context__: HasNameType,
                __Context__: HasField<__Tag__, Value = __Context__::Name>,
            {}
            impl<__Context__, __Provider__> NameGetter<__Context__> for WithProvider<__Provider__>
            where
                __Context__: HasNameType,
                __Provider__: FieldGetter<
                    __Context__,
                    NameGetterComponent,
                    Value = __Context__::Name,
                >,
            {
                fn name(__context__: &__Context__) -> &__Context__::Name {
                    __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<NameGetterComponent>,
                    )
                }
            }
            impl<__Context__, __Provider__> IsProviderFor<NameGetterComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Context__: HasNameType,
                __Provider__: FieldGetter<
                    __Context__,
                    NameGetterComponent,
                    Value = __Context__::Name,
                >,
            {}
            ")
        }
    }

    #[derive(HasField)]
    pub struct MyContext<T> {
        pub name: T,
    }

    snapshot_delegate_and_check_components! {
        delegate_and_check_components! {
            <T>
            MyContext<T> {
                NameTypeProviderComponent: UseType<T>,
                NameGetterComponent: UseField<Symbol!("name")>,
            }
        }

        expand_my_context(output) {
            insta::assert_snapshot!(output, @r#"
            impl<T> DelegateComponent<NameTypeProviderComponent> for MyContext<T> {
                type Delegate = UseType<T>;
            }
            impl<
                T,
                __Context__,
                __Params__,
            > IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for MyContext<T>
            where
                UseType<T>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
            {}
            impl<T> DelegateComponent<NameGetterComponent> for MyContext<T> {
                type Delegate = UseField<Symbol!("name")>;
            }
            impl<
                T,
                __Context__,
                __Params__,
            > IsProviderFor<NameGetterComponent, __Context__, __Params__> for MyContext<T>
            where
                UseField<
                    Symbol!("name"),
                >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
            {}
            trait __CanUseMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl<T> __CanUseMyContext<NameTypeProviderComponent, ()> for MyContext<T> {}
            impl<T> __CanUseMyContext<NameGetterComponent, ()> for MyContext<T> {}
            "#)
        }
    }
}
