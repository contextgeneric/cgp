use core::fmt::Display;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::HandlerRef;
use cgp::prelude::*;
use futures::executor::block_on;

#[cgp_computer]
async fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cgp_computer]
async fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
    a.checked_add(b).ok_or_else(|| "Overflow".to_string())
}

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ErrorTypeProviderComponent:
            UseType<String>,
        ErrorRaiserComponent:
            RaiseFrom,
    }
}

#[test]
fn test_add() {
    let app = App;

    assert_eq!(
        block_on(Add::compute_async(&app, PhantomData::<()>, (1, 2))),
        3,
    );

    assert_eq!(
        block_on(Add::handle(&app, PhantomData::<()>, (1, 2))),
        Ok(3),
    );
}

#[test]
fn test_add_with_error() {
    let app = App;

    assert_eq!(
        block_on(AddWithError::compute_async(&app, PhantomData::<()>, (1, 2))),
        Ok(3),
    );

    assert_eq!(
        block_on(AddWithError::handle(&app, PhantomData::<()>, (1, 2))),
        Ok(3),
    );

    assert_eq!(
        block_on(AddWithError::handle(&app, PhantomData::<()>, (u64::MAX, 1))),
        Err("Overflow".to_string()),
    );
}

#[cgp_computer]
async fn to_string_ref<Value: Display + Sync>(value: &Value) -> String {
    value.to_string()
}

#[test]
fn test_handler_ref() {
    let app = App;
    let code = PhantomData::<()>;

    assert_eq!(block_on(ToStringRef::compute_async(&app, code, &1)), "1");

    assert_eq!(
        block_on(ToStringRef::compute_async_ref(&app, code, &1)),
        "1"
    );

    assert_eq!(
        block_on(ToStringRef::handle(&app, code, &1)),
        Ok("1".to_owned())
    );

    assert_eq!(
        block_on(ToStringRef::handle_ref(&app, code, &1)),
        Ok("1".to_owned())
    );
}
