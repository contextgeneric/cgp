//! Main CGP test suite.
//!
//! All tests live in the integration targets under `tests/`, organized by CGP
//! concept (one target per concept, one file per unit test). This library crate
//! holds the one check that needs a library: `generated_docs`, which denies
//! `missing_docs` so a macro that stops documenting a generated item fails the
//! build. See `../README.md` and `../AGENTS.md` for the organization and the
//! rules for maintaining the suite.

pub mod generated_docs;
