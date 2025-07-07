use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::handler::{ComputerRef, HandlerRef, TryComputerRef};
use cgp::prelude::*;
use futures::executor::block_on;

#[cgp_producer]
pub fn magic_number() -> u64 {
    42
}
#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
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
    assert_eq!(block_on(MagicNumber::handle(&app, code, &input)), Ok(42));
    assert_eq!(
        block_on(MagicNumber::handle_ref(&app, code, &input)),
        Ok(42)
    );
}
