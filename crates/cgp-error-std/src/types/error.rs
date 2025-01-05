use alloc::boxed::Box;
use core::error::Error as StdError;

pub type Error = Box<dyn StdError + Send + Sync + 'static>;
