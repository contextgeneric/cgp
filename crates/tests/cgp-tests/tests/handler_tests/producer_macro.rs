use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{ComputerRef, HandlerRef, TryComputerRef};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;
use futures::executor::block_on;

#[cgp_producer]
pub fn magic_number() -> u64 {
    42
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            ErrorTypeProviderComponent:
                UseType<String>,
        }
    }

    expand_producer_macro_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<ErrorTypeProviderComponent> for App {
            type Delegate = UseType<String>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__> for App
        where
            UseType<String>: IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__>,
        {}
        ")
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
