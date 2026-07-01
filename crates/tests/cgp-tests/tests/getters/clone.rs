//! `#[cgp_getter]` returning an owned `Self::Name` where the associated type is
//! `Copy`: the getter reads the field and `.clone()`s it out by value. The
//! abstract `HasNameType` and the `delegate_components!` wiring are written
//! plainly here (their expansions are owned by the `abstract_types` and
//! `basic_delegation` concepts).
//!
//! See docs/reference/macros/cgp_getter.md and docs/reference/providers/use_field.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasName: HasNameType<Name: Copy> {
        fn name(&self) -> Self::Name;
    }

    expand_has_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasName: HasNameType<Name: Copy> {
            fn name(&self) -> Self::Name;
        }
        impl<__Context__> HasName for __Context__
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: NameGetter<__Context__>,
        {
            fn name(&self) -> Self::Name {
                __Context__::name(self)
            }
        }
        pub trait NameGetter<__Context__>: IsProviderFor<NameGetterComponent, __Context__, ()>
        where
            __Context__: HasNameType<Name: Copy>,
        {
            fn name(__context__: &__Context__) -> __Context__::Name;
        }
        impl<__Provider__, __Context__> NameGetter<__Context__> for __Provider__
        where
            __Context__: HasNameType<Name: Copy>,
            __Provider__: DelegateComponent<NameGetterComponent>
                + IsProviderFor<NameGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                NameGetterComponent,
            >>::Delegate: NameGetter<__Context__>,
        {
            fn name(__context__: &__Context__) -> __Context__::Name {
                <__Provider__ as DelegateComponent<
                    NameGetterComponent,
                >>::Delegate::name(__context__)
            }
        }
        pub struct NameGetterComponent;
        impl<__Context__> NameGetter<__Context__> for UseContext
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasName,
        {
            fn name(__context__: &__Context__) -> __Context__::Name {
                __Context__::name(__context__)
            }
        }
        impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasName,
        {}
        impl<__Context__, __Components__, __Path__> NameGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasNameType<Name: Copy>,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: NameGetter<__Context__>,
        {
            fn name(__context__: &__Context__) -> __Context__::Name {
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
            __Context__: HasNameType<Name: Copy>,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<NameGetterComponent, __Context__, ()>
                + NameGetter<__Context__>,
        {}
        impl<__Context__> NameGetter<__Context__> for UseFields
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = __Context__::Name,
            >,
        {
            fn name(__context__: &__Context__) -> __Context__::Name {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    )
                    .clone()
            }
        }
        impl<__Context__> IsProviderFor<NameGetterComponent, __Context__, ()> for UseFields
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = __Context__::Name,
            >,
        {}
        impl<__Context__, __Tag__> NameGetter<__Context__> for UseField<__Tag__>
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasField<__Tag__, Value = __Context__::Name>,
        {
            fn name(__context__: &__Context__) -> __Context__::Name {
                __context__.get_field(::core::marker::PhantomData::<__Tag__>).clone()
            }
        }
        impl<__Context__, __Tag__> IsProviderFor<NameGetterComponent, __Context__, ()>
        for UseField<__Tag__>
        where
            __Context__: HasNameType<Name: Copy>,
            __Context__: HasField<__Tag__, Value = __Context__::Name>,
        {}
        impl<__Context__, __Provider__> NameGetter<__Context__> for WithProvider<__Provider__>
        where
            __Context__: HasNameType<Name: Copy>,
            __Provider__: FieldGetter<
                __Context__,
                NameGetterComponent,
                Value = __Context__::Name,
            >,
        {
            fn name(__context__: &__Context__) -> __Context__::Name {
                __Provider__::get_field(
                        __context__,
                        ::core::marker::PhantomData::<NameGetterComponent>,
                    )
                    .clone()
            }
        }
        impl<__Context__, __Provider__> IsProviderFor<NameGetterComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Context__: HasNameType<Name: Copy>,
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
pub struct App {
    pub name: &'static str,
}

delegate_components! {
    App {
        NameTypeProviderComponent: UseType<&'static str>,
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}

#[test]
pub fn test_clone_getter() {
    let context = App { name: "Alice" };

    assert_eq!(context.name(), "Alice");
}
