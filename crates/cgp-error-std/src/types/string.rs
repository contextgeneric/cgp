use core::error::Error;
use core::fmt::{Debug, Display};

use alloc::string::String;

pub struct StringError {
    pub message: String,
}

impl From<String> for StringError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl Debug for StringError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl Display for StringError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl Error for StringError {}
