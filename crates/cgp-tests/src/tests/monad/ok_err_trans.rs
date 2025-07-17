use cgp::prelude::*;
// use cgp::extra::monad::monadic::ok::{BindOk, OkMonadic, OkMonadicTrans};

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

// #[test]
// pub fn test_monadic_trans() {
//     let context = ();
//     let code = PhantomData::<()>;

//     assert_eq!(
//         PipeMonadic::<ErrMonadic, Product![ReturnOkOk, ReturnOkOk]>::compute(&context, code, 1),
//         Ok(Ok(1)),
//     );
// }
