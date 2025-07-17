use cgp::extra::handler::PipeHandlers;
use cgp::extra::monad::monadic::ident::IdentMonadic;
use cgp::extra::monad::monadic::ok::{BindOk, OkMonadic};
use cgp::extra::monad::providers::PipeMonadic;
use cgp::prelude::*;

#[cgp_computer]
pub fn increment(value: u8) -> Result<&'static str, u8> {
    match value.checked_add(1) {
        Some(res) => Err(res),
        None => Ok("overflow"),
    }
}

#[test]
fn test_increment_ok() {
    let context = ();
    let code = PhantomData::<()>;

    assert_eq!(Increment::compute(&context, code, 1), Err(2));
    assert_eq!(Increment::compute(&context, code, 255), Ok("overflow"));

    assert_eq!(
        PipeHandlers::<Product![Increment, BindOk<IdentMonadic, Increment>]>::compute(
            &context, code, 1,
        ),
        Err(3),
    );

    assert_eq!(
        PipeHandlers::<Product![Increment, BindOk<IdentMonadic, Increment>]>::compute(
            &context, code, 254,
        ),
        Ok("overflow"),
    );

    assert_eq!(
        PipeMonadic::<OkMonadic, Product![Increment]>::compute(&context, code, 1),
        Err(2),
    );

    assert_eq!(
        PipeMonadic::<OkMonadic, Product![Increment]>::compute(&context, code, 255),
        Ok("overflow"),
    );

    assert_eq!(
        PipeMonadic::<OkMonadic, Product![Increment, Increment, Increment]>::compute(
            &context, code, 253
        ),
        Ok("overflow"),
    );
}
