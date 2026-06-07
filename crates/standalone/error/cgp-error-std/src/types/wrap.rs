use alloc::string::String;
use core::error::Error as StdError;
use core::fmt::{Debug, Display};

use crate::Error;

pub struct WrapError {
    pub detail: String,
    pub source: Error,
}

impl Display for WrapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.detail, self.source)
    }
}

impl Debug for WrapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self, f)
    }
}

impl StdError for WrapError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }
}
