use cgp_macro_core::types::cgp_provider::ItemCgpProvider;
use cgp_macro_core::types::delegate_component::DelegateTable;
use syn::ItemFn;

/// The intermediate representation `#[cgp_computer]` and `#[cgp_producer]`
/// evaluate a function into, built from `cgp-macro-core` AST nodes rather than
/// emitted as nested macro invocations.
///
/// Each field is what the macro would otherwise write out for another macro to
/// expand: `provider` is the `#[cgp_new_provider] impl …` block, and
/// `delegate_table` is the `delegate_components! { … }` body that promotes the
/// provider across the handler family. The entrypoint lowers both with
/// [`ItemCgpProvider::lower`] and [`DelegateTable::eval`].
pub struct EvaluatedHandlerFn {
    /// The annotated function, emitted unchanged.
    pub item_fn: ItemFn,
    /// The base provider impl, which declares the provider struct.
    pub provider: ItemCgpProvider,
    /// The wiring that routes the rest of the handler family to a promotion
    /// bundle.
    pub delegate_table: DelegateTable,
}
