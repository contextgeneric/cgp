use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_getter, snapshot_delegate_components};

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasName {
        type Name: Display;

        fn name(&self) -> &Self::Name;
    }

    expand_has_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasName {
            type Name: Display;
            fn name(&self) -> &Self::Name;
        }
        impl<__Context__> HasName for __Context__
        where
            __Context__: NameGetter<__Context__>,
        {
            type Name = <__Context__ as NameGetter<__Context__>>::Name;
            fn name(&self) -> &Self::Name {
                __Context__::name(self)
            }
        }
        pub trait NameGetter<__Context__>: IsProviderFor<NameGetterComponent, __Context__, ()> {
            type Name: Display;
            fn name(__context__: &__Context__) -> &Self::Name;
        }
        impl<__Provider__, __Context__> NameGetter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                NameGetterComponent,
            >>::Delegate: NameGetter<__Context__>,
        {
            type Name = <<__Provider__ as DelegateComponent<
                NameGetterComponent,
            >>::Delegate as NameGetter<__Context__>>::Name;
            fn name(__context__: &__Context__) -> &Self::Name {
                <__Provider__ as DelegateComponent<
                    NameGetterComponent,
                >>::Delegate::name(__context__)
            }
        }
        pub struct NameGetterComponent;
        impl<__Context__> NameGetter<__Context__> for UseContext
        where
            __Context__: HasName,
        {
            type Name = <__Context__ as HasName>::Name;
            fn name(__context__: &__Context__) -> &Self::Name {
                __Context__::name(__context__)
            }
        }
        impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasName,
        {}
        impl<__Context__, __Components__, __Path__> NameGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: NameGetter<__Context__>,
        {
            type Name = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as NameGetter<__Context__>>::Name;
            fn name(__context__: &__Context__) -> &Self::Name {
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
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<NameGetterComponent, __Context__, ()>
                + NameGetter<__Context__>,
        {}
        impl<__Context__, Name> NameGetter<__Context__> for UseFields
        where
            Name: Display,
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = Name,
            >,
        {
            type Name = Name;
            fn name(__context__: &__Context__) -> &Self::Name {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
            }
        }
        impl<__Context__, Name> IsProviderFor<NameGetterComponent, __Context__, ()> for UseFields
        where
            Name: Display,
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = Name,
            >,
        {}
        impl<__Context__, Name, __Tag__> NameGetter<__Context__> for UseField<__Tag__>
        where
            Name: Display,
            __Context__: HasField<__Tag__, Value = Name>,
        {
            type Name = Name;
            fn name(__context__: &__Context__) -> &Self::Name {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>)
            }
        }
        impl<__Context__, Name, __Tag__> IsProviderFor<NameGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            Name: Display,
            __Context__: HasField<__Tag__, Value = Name>,
        {}
        impl<__Context__, Name, __Provider__> NameGetter<__Context__>
        for WithProvider<__Provider__>
        where
            Name: Display,
            __Provider__: FieldGetter<__Context__, NameGetterComponent, Value = Name>,
        {
            type Name = Name;
            fn name(__context__: &__Context__) -> &Self::Name {
                __Provider__::get_field(
                    __context__,
                    ::core::marker::PhantomData::<NameGetterComponent>,
                )
            }
        }
        impl<__Context__, Name, __Provider__> IsProviderFor<NameGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            Name: Display,
            __Provider__: FieldGetter<__Context__, NameGetterComponent, Value = Name>,
        {}
        ")
    }
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String,
}

snapshot_delegate_components! {
    delegate_components! {
        Person {
            NameGetterComponent:
                UseField<Symbol!("first_name")>,
        }
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @r#"
        impl DelegateComponent<NameGetterComponent> for Person {
            type Delegate = UseField<Symbol!("first_name")>;
        }
        impl<__Context__, __Params__> IsProviderFor<NameGetterComponent, __Context__, __Params__>
        for Person
        where
            UseField<
                Symbol!("first_name"),
            >: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
        {}
        "#)
    }
}

pub trait CheckHasName: HasName<Name = String> {}
impl CheckHasName for Person {}
