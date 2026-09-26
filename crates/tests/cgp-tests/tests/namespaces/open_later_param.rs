//! `open` dispatch on a component's second type parameter alone.
//!
//! The redirect appends every type parameter to the lookup path, so a lookup of
//! `CanDescribe<Code, Input>` follows `@DescriberComponent.Code.Input`. A key whose
//! first segment is a per-entry generic, `@DescriberComponent.<Code> Code.u64`,
//! matches every `Code` and dispatches on the input, which is the `open` form of a
//! `UseInputDelegate` table. The test calls one context with two codes per input
//! and checks that the input alone selects the provider. The `delegate_components!`
//! snapshot pins the generic-first-segment expansion; the component and providers
//! are incidental scaffolding.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/delegate_components.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// Incidental: a component with a code parameter and an input parameter.
#[cgp_component(Describer)]
pub trait CanDescribe<Code, Input> {
    fn describe(&self, code: PhantomData<Code>, input: Input) -> String;
}

// Incidental: one provider per input type, each generic over the code.
#[cgp_impl(new DescribeNumber)]
impl<Code> Describer<Code, u64> {
    fn describe(&self, _code: PhantomData<Code>, input: u64) -> String {
        format!("number {input}")
    }
}

#[cgp_impl(new DescribeText)]
impl<Code> Describer<Code, String> {
    fn describe(&self, _code: PhantomData<Code>, input: String) -> String {
        format!("text {input}")
    }
}

pub struct Short;

pub struct Long;

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            open DescriberComponent;

            @DescriberComponent.<Code> Code.u64:
                DescribeNumber,
            @DescriberComponent.<Code> Code.String:
                DescribeText,
        }
    }

    expand_open_later_param(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<DescriberComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<DescriberComponent, Nil>>;
        }
        impl<__Context__, __Params__> IsProviderFor<DescriberComponent, __Context__, __Params__>
        for App
        where
            RedirectLookup<
                App,
                PathCons<DescriberComponent, Nil>,
            >: IsProviderFor<DescriberComponent, __Context__, __Params__>,
        {}
        impl<
            Code,
            __Wildcard__,
        > DelegateComponent<
            PathCons<DescriberComponent, PathCons<Code, PathCons<u64, __Wildcard__>>>,
        > for App {
            type Delegate = DescribeNumber;
        }
        impl<
            Code,
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<DescriberComponent, PathCons<Code, PathCons<u64, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for App
        where
            DescribeNumber: IsProviderFor<
                PathCons<DescriberComponent, PathCons<Code, PathCons<u64, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            Code,
            __Wildcard__,
        > DelegateComponent<
            PathCons<DescriberComponent, PathCons<Code, PathCons<String, __Wildcard__>>>,
        > for App {
            type Delegate = DescribeText;
        }
        impl<
            Code,
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<DescriberComponent, PathCons<Code, PathCons<String, __Wildcard__>>>,
            __Context__,
            __Params__,
        > for App
        where
            DescribeText: IsProviderFor<
                PathCons<DescriberComponent, PathCons<Code, PathCons<String, __Wildcard__>>>,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    App {
        DescriberComponent: [
            (Short, u64),
            (Long, u64),
            (Short, String),
            (Long, String),
        ],
    }
}

#[test]
fn test_open_later_param() {
    let app = App;

    assert_eq!(app.describe(PhantomData::<Short>, 7), "number 7");
    assert_eq!(app.describe(PhantomData::<Long>, 7), "number 7");
    assert_eq!(
        app.describe(PhantomData::<Short>, "hi".to_owned()),
        "text hi"
    );
    assert_eq!(
        app.describe(PhantomData::<Long>, "hi".to_owned()),
        "text hi"
    );
}
