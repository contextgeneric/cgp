/// The parsed modifier attributes.
pub mod attributes;
/// The AST for `#[blanket_trait]`.
pub mod blanket_trait;
/// The AST for `#[cgp_auto_getter]`.
pub mod cgp_auto_getter;
/// The AST for `#[cgp_component]`.
pub mod cgp_component;
/// The AST for the extensible-data derives.
pub mod cgp_data;
/// The AST for `#[cgp_fn]`.
pub mod cgp_fn;
/// The AST for `#[cgp_getter]`.
pub mod cgp_getter;
/// The AST for `#[cgp_impl]`.
pub mod cgp_impl;
/// The AST for `#[cgp_provider]` and `#[cgp_new_provider]`.
pub mod cgp_provider;
/// The AST for `#[cgp_type]`.
pub mod cgp_type;
/// The AST for `check_components!`.
pub mod check_components;
/// The AST for `delegate_and_check_components!`.
pub mod delegate_and_check_components;
/// The AST for `delegate_components!`.
pub mod delegate_component;
/// The AST for a generated empty struct.
pub mod empty_struct;
/// The AST for field access and bounds.
pub mod field;
/// The AST for generic parameters.
pub mod generics;
/// The AST for getter methods.
pub mod getter;
/// The AST for identifiers and their generics.
pub mod ident;
/// The AST for `#[implicit]` arguments.
pub mod implicits;
/// The AST for keyword markers.
pub mod keyword;
/// The keyword markers the parsers use.
pub mod keywords;
/// The AST for `cgp_namespace!`.
pub mod namespace;
/// The AST for type-level paths.
pub mod path;
/// The AST for `Product!`.
pub mod product;
/// The AST for a provider impl.
pub mod provider_impl;
/// The AST for struct and enum shapes.
pub mod shape;
/// The AST for `Sum!`.
pub mod sum;
