use core::fmt::Display;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::{ComputerRef, HandlerRef, TryComputerRef};
use cgp::prelude::*;
use futures::executor::block_on;

#[cgp_computer]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cgp_computer]
fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
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
fn test_generated_computers() {
    let app = App;

    assert_eq!(Add::compute(&app, PhantomData::<()>, (1, 2)), 3,);

    assert_eq!(Add::try_compute(&app, PhantomData::<()>, (1, 2)), Ok(3),);

    assert_eq!(
        AddWithError::try_compute(&app, PhantomData::<()>, (1, 2)),
        Ok(3),
    );

    assert_eq!(
        AddWithError::try_compute(&app, PhantomData::<()>, (u64::MAX, 1)),
        Err("Overflow".to_string()),
    );

    assert_eq!(
        block_on(Add::handle(&app, PhantomData::<()>, (1, 2))),
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
fn to_string_ref<Value: Display>(value: &Value) -> String {
    value.to_string()
}

#[test]
fn test_computer_ref() {
    let app = App;
    let code = PhantomData::<()>;

    assert_eq!(ToStringRef::compute(&app, code, &1), "1");
    assert_eq!(ToStringRef::compute_ref(&app, code, &1), "1");
    assert_eq!(ToStringRef::try_compute(&app, code, &1).unwrap(), "1");
    assert_eq!(ToStringRef::try_compute_ref(&app, code, &1).unwrap(), "1");
    assert_eq!(block_on(ToStringRef::handle(&app, code, &1)).unwrap(), "1");
    assert_eq!(
        block_on(ToStringRef::handle_ref(&app, code, &1)).unwrap(),
        "1"
    );
}
