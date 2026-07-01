//! Composing `Computer` handlers through the **err monad** — the dual of the ok
//! monad.
//!
//! `ErrMonadic` treats `Result<T, E>` as a monad over its `Err` value: chaining
//! handlers threads each `Err` result into the next step and short-circuits on
//! the first `Ok`. The `increment` computer here uses the ordinary convention
//! (`Ok(next)` on success, `Err("overflow")` on failure). Both
//! `BindErr<IdentMonadic, ..>` (inside a plain `PipeHandlers` chain) and the
//! `PipeMonadic<ErrMonadic, ..>` combinator are exercised.
//!
//! See docs/concepts/monadic-handlers.md and
//! docs/reference/providers/monad_providers.md.

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
