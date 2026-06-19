use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;
use cgp_tests::namespaces::extended::ExtendedNamespace;

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace ExtendedNamespace;

            @app.ErrorTypeProviderComponent:
                UseType<String>,
            @app.{
                ErrorRaiserComponent.{&'static str, String},
                ErrorWrapperComponent,
            }:
                RaiseFrom,
            TryComputerComponent:
                Foo,
        }
    }

    expand_extended_ns_app(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for App
        where
            __Key__: ExtendedNamespace<App, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for App
        where
            __Key__: ExtendedNamespace<App, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorTypeProviderComponent, __Wildcard__>,
            >,
        > for App {
            type Delegate = UseType<String>;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorTypeProviderComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            UseType<
                String,
            >: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorTypeProviderComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorWrapperComponent, __Wildcard__>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorWrapperComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorWrapperComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl DelegateComponent<TryComputerComponent> for App {
            type Delegate = Foo;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for App
        where
            Foo: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        ")
    }
}

#[cgp_computer]
fn foo(x: u64) -> Result<u64, String> {
    Ok(x * 2)
}

pub trait CheckApp: HasErrorType + CanRaiseError<&'static str> + CanTryCompute<(), u64> {}

impl CheckApp for App {}
