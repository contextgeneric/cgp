use alloc::string::String;
use core::error::Error as StdError;
use core::fmt::{Debug, Display, Formatter, Result};

use crate::Error;

/// A standard error that adds a detail message to the error it wraps, which it returns as its
/// `source`.
///
/// `{}` prints the detail alone, so a reporter that walks `source()` prints each message once.
/// `{:#}` and `{:?}` print the whole chain, joined by `": "`.
pub struct WrapError {
    pub detail: String,
    pub source: Error,
}

impl WrapError {
    fn fmt_chain(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.detail)?;

        let mut source: Option<&(dyn StdError + 'static)> = Some(self.source.as_ref());
        while let Some(error) = source {
            write!(f, ": {error}")?;
            source = error.source();
        }

        Ok(())
    }
}

impl Display for WrapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            self.fmt_chain(f)
        } else {
            f.write_str(&self.detail)
        }
    }
}

impl Debug for WrapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt_chain(f)
    }
}

impl StdError for WrapError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }
}
