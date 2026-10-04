//! `#[cgp_producer]`: turning an input-free function into a `Producer` provider.
//!
//! A function with no input annotated with `#[cgp_producer]` becomes a
//! `Producer` — production driven by only the context and a `Code` tag. CGP
//! promotes it across the rest of the computation family, so the same definition
//! is callable as `produce`, `compute`, `try_compute`, `compute_async`, and
//! `handle` (plus the by-ref variants); the promoted forms simply ignore the
//! input value.
//!
//! The error type wiring on `App` is incidental scaffolding, so it uses the
//! plain `delegate_components!`.
//!
//! This target owns the `#[cgp_producer]` snapshot, pinned here.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_producer.md and
//! cgp-knowledge-base/cgp/reference/components/producer.md.

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{ComputerRef, HandlerRef, TryComputerRef};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_producer;
use futures::executor::block_on;

snapshot_cgp_producer! {
    #[cgp_producer]
    pub fn magic_number() -> u64 {
        42
    }

    expand_magic_number(output) {
        insta::assert_snapshot!(output, @"
        pub fn magic_number() -> u64 {
            42
        }
        impl<__Context__, __Code__> Producer<__Context__, __Code__> for MagicNumber {
            type Output = u64;
            fn produce(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
            ) -> Self::Output {
                magic_number()
            }
        }
        impl<__Context__, __Code__> IsProviderFor<ProducerComponent, __Context__, (__Code__)>
        for MagicNumber {}
        pub struct MagicNumber;
        impl DelegateComponent<ComputerComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<Self>: IsProviderFor<ComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<ComputerRefComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<Self>: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<Self>: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<
                Self,
            >: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<
                Self,
            >: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<
                Self,
            >: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<Self>: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for MagicNumber {
            type Delegate = PromoteProducer<Self>;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for MagicNumber
        where
            PromoteProducer<Self>: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseType<String>,
    }
}

#[test]
fn test_producer_macro() {
    let app = App;
    let code = PhantomData::<()>;
    let input = ();

    assert_eq!(MagicNumber::produce(&app, code), 42);

    assert_eq!(MagicNumber::compute(&app, code, &input), 42);

    assert_eq!(MagicNumber::compute_ref(&app, code, &input), 42);

    assert_eq!(MagicNumber::try_compute(&app, code, &input), Ok(42));

    assert_eq!(MagicNumber::try_compute_ref(&app, code, &input), Ok(42));

    assert_eq!(block_on(MagicNumber::compute_async(&app, code, &input)), 42);

    assert_eq!(
        block_on(MagicNumber::compute_async_ref(&app, code, &input)),
        42
    );

    assert_eq!(block_on(MagicNumber::handle(&app, code, &input)), Ok(42));

    assert_eq!(
        block_on(MagicNumber::handle_ref(&app, code, &input)),
        Ok(42)
    );
}
