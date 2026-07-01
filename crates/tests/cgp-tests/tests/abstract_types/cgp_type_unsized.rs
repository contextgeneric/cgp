//! `#[cgp_type]` with an unsized (`T: ?Sized`) generic parameter and a custom
//! provider name.
//!
//! This variant exercises two things at once: the `#[cgp_type(ProvideFooType)]`
//! argument overriding the default `…TypeProvider` name, and a `?Sized` generic
//! parameter, which the expansion threads through every generated item (the
//! provider trait's `Params` tuple becomes `(T)` and the `RedirectLookup` impls
//! extend the path with `PathCons<T, Nil>`). A trailing `#[cgp_getter]` that
//! depends on the abstract type is incidental scaffolding and is written plainly.
//!
//! See docs/reference/macros/cgp_type.md and docs/concepts/abstract-types.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_type;

snapshot_cgp_type! {
    #[cgp_type(ProvideFooType)]
    pub trait HasFooType<T: ?Sized> {
        type Foo;
    }

    expand_has_foo_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasFooType<T: ?Sized> {
            type Foo;
        }
        impl<__Context__, T: ?Sized> HasFooType<T> for __Context__
        where
            __Context__: ProvideFooType<__Context__, T>,
        {
            type Foo = <__Context__ as ProvideFooType<__Context__, T>>::Foo;
        }
        pub trait ProvideFooType<
            __Context__,
            T: ?Sized,
        >: IsProviderFor<ProvideFooTypeComponent, __Context__, (T)> {
            type Foo;
        }
        impl<__Provider__, __Context__, T: ?Sized> ProvideFooType<__Context__, T>
        for __Provider__
        where
            __Provider__: DelegateComponent<ProvideFooTypeComponent>
                + IsProviderFor<ProvideFooTypeComponent, __Context__, (T)>,
            <__Provider__ as DelegateComponent<
                ProvideFooTypeComponent,
            >>::Delegate: ProvideFooType<__Context__, T>,
        {
            type Foo = <<__Provider__ as DelegateComponent<
                ProvideFooTypeComponent,
            >>::Delegate as ProvideFooType<__Context__, T>>::Foo;
        }
        pub struct ProvideFooTypeComponent;
        impl<__Context__, T: ?Sized> ProvideFooType<__Context__, T> for UseContext
        where
            __Context__: HasFooType<T>,
        {
            type Foo = <__Context__ as HasFooType<T>>::Foo;
        }
        impl<__Context__, T: ?Sized> IsProviderFor<ProvideFooTypeComponent, __Context__, (T)>
        for UseContext
        where
            __Context__: HasFooType<T>,
        {}
        impl<__Context__, T: ?Sized, __Components__, __Path__> ProvideFooType<__Context__, T>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: ProvideFooType<__Context__, T>,
        {
            type Foo = <<__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate as ProvideFooType<__Context__, T>>::Foo;
        }
        impl<
            __Context__,
            T: ?Sized,
            __Components__,
            __Path__,
        > IsProviderFor<ProvideFooTypeComponent, __Context__, (T)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<T, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<T, Nil>>>::Output,
            >>::Delegate: IsProviderFor<ProvideFooTypeComponent, __Context__, (T)>
                + ProvideFooType<__Context__, T>,
        {}
        impl<Foo, __Context__, T: ?Sized> ProvideFooType<__Context__, T> for UseType<Foo> {
            type Foo = Foo;
        }
        impl<
            Foo,
            __Context__,
            T: ?Sized,
        > IsProviderFor<ProvideFooTypeComponent, __Context__, (T)> for UseType<Foo> {}
        impl<__Provider__, Foo, __Context__, T: ?Sized> ProvideFooType<__Context__, T>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ProvideFooTypeComponent, Type = Foo>,
        {
            type Foo = Foo;
        }
        impl<
            __Provider__,
            Foo,
            __Context__,
            T: ?Sized,
        > IsProviderFor<ProvideFooTypeComponent, __Context__, (T)> for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ProvideFooTypeComponent, Type = Foo>,
        {}
        ")
    }
}

#[cgp_getter]
pub trait HasFoo<T: ?Sized>: HasFooType<T> {
    fn foo(&self) -> &Self::Foo;
}
