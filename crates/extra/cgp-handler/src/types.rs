use core::marker::PhantomData;

#[allow(non_upper_case_globals)]
/// A code tag for a computation that takes no code.
pub const NoCode: PhantomData<()> = PhantomData;

/// A provider that dispatches a computation on its input type.
///
/// See [`UseInputDelegate`](https://contextgeneric.dev/docs/reference/providers/handler/use_input_delegate).
pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
