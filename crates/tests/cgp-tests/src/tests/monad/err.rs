use cgp::extra::handler::PipeHandlers;
use cgp::extra::monad::monadic::err::{BindErr, ErrMonadic};
use cgp::extra::monad::monadic::ident::IdentMonadic;
use cgp::extra::monad::providers::PipeMonadic;
use cgp::prelude::*;

#[cgp_computer]
pub fn increment(value: u8) -> Result<u8, &'static str> {
    value.checked_add(1).ok_or("overflow")
}

#[test]
fn test_increment() {
    let context = ();
    let code = PhantomData::<()>;

    assert_eq!(Increment::compute(&context, code, 1), Ok(2));
    assert_eq!(Increment::compute(&context, code, 255), Err("overflow"));

    assert_eq!(
        PipeHandlers::<Product![Increment, BindErr<IdentMonadic, Increment>]>::compute(
            &context, code, 1,
        ),
        Ok(3),
    );

    assert_eq!(
        PipeHandlers::<Product![Increment, BindErr<IdentMonadic, Increment>]>::compute(
            &context, code, 254,
        ),
        Err("overflow"),
    );

    assert_eq!(
        PipeMonadic::<ErrMonadic, Product![Increment]>::compute(&context, code, 1),
        Ok(2),
    );

    assert_eq!(
        PipeMonadic::<ErrMonadic, Product![Increment]>::compute(&context, code, 255),
        Err("overflow"),
    );

    assert_eq!(
        PipeMonadic::<ErrMonadic, Product![Increment, Increment, Increment]>::compute(
            &context, code, 253
        ),
        Err("overflow"),
    );
}
