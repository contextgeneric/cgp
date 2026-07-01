//! One unit test per file. Each file is self-contained: it defines its own
//! `#[cgp_computer]` handlers and drives them through the monadic combinators
//! at module scope, so the wiring of one test never leaks into another.

// The ok monad: `PipeMonadic<OkMonadic, ..>` and `BindOk` sequence handlers,
// short-circuiting on the first `Err` and threading each `Ok` value onward.
pub mod ok_monadic;

// The err monad: the dual of the ok monad — `PipeMonadic<ErrMonadic, ..>` and
// `BindErr` thread the `Err` value onward and short-circuit on the first `Ok`.
pub mod err_monadic;

// The ok-monad transformer `OkMonadicTrans<Inner>` stacked over `ErrMonadic`,
// plus running the same pipeline via `try_compute` and the async `handle`.
pub mod ok_err_monadic_trans;
