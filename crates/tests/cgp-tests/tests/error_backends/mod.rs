//! One unit test per file. Each file is self-contained: it defines its own
//! contexts and wiring at module scope so that the type-level wiring of one test
//! never leaks into another.

pub mod anyhow_formatting;
pub mod anyhow_raise_and_wrap;
pub mod eyre_formatting;
pub mod eyre_location;
pub mod eyre_raise_and_wrap;
pub mod generic_equivalents;
pub mod namespace_wiring;
pub mod readme_anyhow;
pub mod readme_eyre;
pub mod readme_std;
pub mod std_formatting;
pub mod std_raise_and_wrap;
pub mod std_wrap_error;
pub mod swapping_backends;
