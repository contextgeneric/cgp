//! `#[cgp_component]` on a trait with a lifetime and a type parameter.
//!
//! When a component carries a lifetime, the generated provider trait keeps the
//! lifetime ahead of `__Context__` and the lifetime is lifted into `Life<'a>` in
//! the `IsProviderFor` params tuple (`(Life<'a>, T)`); the `RedirectLookup` impl
//! appends the type parameter to the lookup path via `ConcatPath`. This is the
//! reference snapshot for the lifetime/type-parameter variant of the macro, plus
//! the matching lifetime-carrying `UseField` provider. The `#[cgp_impl]`,
//! `delegate_components!`, and `check_components!` wiring below is written plainly
//! (its expansion is owned by the `basic_delegation` and `checking` concepts).
//!
//! See docs/reference/macros/cgp_component.md and docs/reference/types/life.md.

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_component, snapshot_cgp_provider};

snapshot_cgp_component! {
    #[cgp_component(ReferenceGetter)]
    pub trait HasReference<'a, T: 'a + ?Sized> {
        fn get_reference(&self) -> &'a T;
    }

    expand_can_greet(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasReference<'a, T: 'a + ?Sized> {
            fn get_reference(&self) -> &'a T;
        }
        impl<'a, __Context__, T: 'a + ?Sized> HasReference<'a, T> for __Context__
        where
            __Context__: ReferenceGetter<'a, __Context__, T>,
        {
            fn get_reference(&self) -> &'a T {
                __Context__::get_reference(self)
            }
        }
        pub trait ReferenceGetter<
            'a,
            __Context__,
            T: 'a + ?Sized,
        >: IsProviderFor<ReferenceGetterComponent, __Context__, (Life<'a>, T)> {
            fn get_reference(__context__: &__Context__) -> &'a T;
        }
        impl<'a, __Provider__, __Context__, T: 'a + ?Sized> ReferenceGetter<'a, __Context__, T>
        for __Provider__
        where
            __Provider__: DelegateComponent<ReferenceGetterComponent>
                + IsProviderFor<ReferenceGetterComponent, __Context__, (Life<'a>, T)>,
            <__Provider__ as DelegateComponent<
                ReferenceGetterComponent,
            >>::Delegate: ReferenceGetter<'a, __Context__, T>,
        {
            fn get_reference(__context__: &__Context__) -> &'a T {
                <__Provider__ as DelegateComponent<
                    ReferenceGetterComponent,
                >>::Delegate::get_reference(__context__)
            }
        }
        pub struct ReferenceGetterComponent;
        impl<'a, __Context__, T: 'a + ?Sized> ReferenceGetter<'a, __Context__, T> for UseContext
        where
            __Context__: HasReference<'a, T>,
        {
            fn get_reference(__context__: &__Context__) -> &'a T {
                __Context__::get_reference(__context__)
            }
        }
        impl<
            'a,
            __Context__,
            T: 'a + ?Sized,
        > IsProviderFor<ReferenceGetterComponent, __Context__, (Life<'a>, T)> for UseContext
        where
            __Context__: HasReference<'a, T>,
        {}
        impl<
            'a,
            __Context__,
            T: 'a + ?Sized,
            __Components__,
            __Path__,
        > ReferenceGetter<'a, __Context__, T> for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: ReferenceGetter<'a, __Context__, T>,
        {
            fn get_reference(__context__: &__Context__) -> &'a T {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
                >>::Delegate::get_reference(__context__)
            }
        }
        impl<
            'a,
            __Context__,
            T: 'a + ?Sized,
            __Components__,
            __Path__,
        > IsProviderFor<ReferenceGetterComponent, __Context__, (Life<'a>, T)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: ReferenceGetter<'a, __Context__, T>,
        {}
        ")
    }
}

snapshot_cgp_provider! {
    #[cgp_provider]
    impl<'a, Context, Tag, T: 'a + ?Sized> ReferenceGetter<'a, Context, T> for UseField<Tag>
    where
        Context: HasField<Tag, Value = &'a T>,
    {
        fn get_reference(context: &Context) -> &'a T {
            context.get_field(PhantomData)
        }
    }

    expand_use_field_reference_getter(output) {
        insta::assert_snapshot!(output, @"
        impl<'a, Context, Tag, T: 'a + ?Sized> ReferenceGetter<'a, Context, T> for UseField<Tag>
        where
            Context: HasField<Tag, Value = &'a T>,
        {
            fn get_reference(context: &Context) -> &'a T {
                context.get_field(PhantomData)
            }
        }
        impl<
            'a,
            Context,
            Tag,
            T: 'a + ?Sized,
        > IsProviderFor<ReferenceGetterComponent, Context, (Life<'a>, T)> for UseField<Tag>
        where
            Context: HasField<Tag, Value = &'a T>,
        {}
        ")
    }
}

pub struct App<'a> {
    pub value: &'a str,
}

#[cgp_impl(new GetReference)]
impl<'a> ReferenceGetter<'a, str> for App<'a> {
    fn get_reference(&self) -> &'a str {
        self.value
    }
}

delegate_components! {
    <'a> App<'a> {
        ReferenceGetterComponent:
            GetReference,
    }
}

check_components! {
    <'a> App<'a> {
        ReferenceGetterComponent:
            (Life<'a>, str),
    }
}
