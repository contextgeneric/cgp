use cgp::extra::monad::monadic::err::{BindErr, ErrMonadic};
use cgp::extra::monad::monadic::ident::IdentMonadic;
use cgp::extra::monad::monadic::ok::{BindOk, OkMonadicTrans};
use cgp::extra::monad::providers::PipeMonadic;
use cgp::prelude::*;

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

pub trait CheckImpl:
    Computer<
    (),
    (),
    Result<Result<(), u8>, &'static str>,
    Output = Result<Result<(), u8>, &'static str>,
>
{
}
impl CheckImpl for BindErr<IdentMonadic, BindOk<ErrMonadic, ReturnOkErr>> {}

#[test]
pub fn test_ok_err_monadic_trans() {
    let context = ();
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
}
