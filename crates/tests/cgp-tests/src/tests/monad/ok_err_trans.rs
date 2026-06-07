use cgp::core::error::ErrorOnly;
use cgp::extra::monad::monadic::err::ErrMonadic;
use cgp::extra::monad::monadic::ok::{OkMonadic, OkMonadicTrans};
use cgp::extra::monad::providers::PipeMonadic;
use cgp::prelude::*;
use futures::executor::block_on;

#[cgp_computer]
pub fn return_ok_ok(_value: u8) -> Result<Result<(), u8>, &'static str> {
    Ok(Ok(()))
}

#[cgp_computer]
pub fn return_ok_err(value: u8) -> Result<Result<(), u8>, &'static str> {
    Ok(Err(value))
}

#[cgp_computer]
pub fn return_err(_value: u8) -> Result<Result<(), u8>, &'static str> {
    Err("error")
}

#[test]
pub fn test_ok_err_monadic_trans() {
    let context = ErrorOnly::<&'static str>::default();
    let code = PhantomData::<()>;

    assert_eq!(
        PipeMonadic::<OkMonadicTrans<ErrMonadic>, Product![ReturnOkErr, ReturnOkErr, ReturnOkErr]>::compute(
            &context, code, 1
        ),
        Ok(Err(1)),
    );

    assert_eq!(
        PipeMonadic::<OkMonadicTrans<ErrMonadic>, Product![ReturnOkErr, ReturnOkOk, ReturnOkErr]>::compute(
            &context, code, 1
        ),
        Ok(Ok(())),
    );

    assert_eq!(
        PipeMonadic::<OkMonadicTrans<ErrMonadic>, Product![ReturnErr, ReturnOkOk, ReturnOkErr]>::compute(
            &context, code, 1
        ),
        Err("error"),
    );

    assert_eq!(
        PipeMonadic::<OkMonadic, Product![ReturnOkErr, ReturnOkErr, ReturnOkErr]>::try_compute(
            &context, code, 1
        ),
        Ok(Err(1)),
    );

    assert_eq!(
        PipeMonadic::<OkMonadic, Product![ReturnOkErr, ReturnOkOk, ReturnOkErr]>::try_compute(
            &context, code, 1
        ),
        Ok(Ok(())),
    );

    assert_eq!(
        PipeMonadic::<OkMonadic, Product![ReturnErr, ReturnOkOk, ReturnOkErr]>::try_compute(
            &context, code, 1
        ),
        Err("error"),
    );

    assert_eq!(
        block_on(PipeMonadic::<
            OkMonadic,
            Product![ReturnOkErr, ReturnOkErr, ReturnOkErr],
        >::handle(&context, code, 1)),
        Ok(Err(1)),
    );

    assert_eq!(
        block_on(PipeMonadic::<
            OkMonadic,
            Product![ReturnOkErr, ReturnOkOk, ReturnOkErr],
        >::handle(&context, code, 1)),
        Ok(Ok(())),
    );

    assert_eq!(
        block_on(PipeMonadic::<
            OkMonadic,
            Product![ReturnErr, ReturnOkOk, ReturnOkErr],
        >::handle(&context, code, 1)),
        Err("error"),
    );
}
