use core::marker::PhantomData;

/// A code tag for a computation that takes no code.
#[allow(non_upper_case_globals)]
pub const NoCode: PhantomData<()> = PhantomData;

/// The legacy provider that routes a handler component to a per-input-type provider through a
/// nested table; the open statement replaces it.
///
/// See
/// [`UseInputDelegate`](https://contextgeneric.dev/docs/reference/providers/handler/use_input_delegate)
/// .
pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
