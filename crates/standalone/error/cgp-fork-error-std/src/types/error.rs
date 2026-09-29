use alloc::boxed::Box;
use core::error::Error as StdError;

/// The abstract error type [`UseBoxedStdError`](crate::UseBoxedStdError) selects: any standard
/// error, boxed.
pub type Error = Box<dyn StdError + Send + Sync + 'static>;
