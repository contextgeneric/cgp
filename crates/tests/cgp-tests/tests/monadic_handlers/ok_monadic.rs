//! Composing `Computer` handlers through the **ok monad**.
//!
//! `OkMonadic` treats `Result<T, E>` as a monad over its `Ok` value: chaining
//! handlers threads each `Ok` result into the next step and short-circuits on
//! the first `Err`. Here the `increment` computer inverts the usual convention
//! (it returns `Err(res)` on success so the `Ok` value can carry the terminal
//! "overflow" string), which makes the ok-monad's short-circuit-on-`Err`
//! behavior easy to observe. Both `BindOk<IdentMonadic, ..>` (inside a plain
//! `PipeHandlers` chain) and the `PipeMonadic<OkMonadic, ..>` combinator are
//! exercised.
//!
//! See docs/concepts/monadic-handlers.md and
//! docs/reference/providers/monad_providers.md.

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
