//! Entrypoint for parser tests of the `Struct!` and `Enum!` shape macros in
//! `cgp-fork-macro-core`.
//!
//! These call the `StructType` and `EnumType` parsers directly to pin which
//! form each body is read as. A proc macro never sees the delimiter it was
//! invoked with, so the named or tuple form of a `Struct!` body is decided from
//! its entries, and a misread form would otherwise surface only as an opaque
//! type mismatch downstream.
#![allow(dead_code)]

pub mod shape_macro_parsing;
