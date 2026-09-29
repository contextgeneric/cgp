use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::In;
use syn::{Generics, ItemImpl, Type};

use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::ident::PathWithTypeArgs;
use crate::types::path::UniPathOrType;

pub struct DefaultImplAttribute {
    pub key_type: UniPathOrType,
    pub in_token: In,
    pub namespace: PathWithTypeArgs,
}

impl DefaultImplAttribute {
    pub fn to_item_impl(
        &self,
        provider_generics: &Generics,
        provider_type: &Type,
    ) -> syn::Result<ItemImpl> {
        let key_type = &self.key_type;
        let mut namespace_trait_path = self.namespace.clone();

        namespace_trait_path
            .type_args
            .args
            .push(parse_internal!(__Components__));

        let mut generics = provider_generics.clone();

        // Drop the provider's impl-side dependencies. `provider_generics` is the
        // provider impl's generics *after* `#[implicit]`/`#[uses]`/`#[use_type]`/
        // `#[use_provider]` have pushed their `Self`-keyed bounds into its `where`
        // clause (e.g. `Self: HasErrorType`). Those belong on the provider's own
        // impl and its `IsProviderFor`, never on this registration impl, whose only
        // job is `type Delegate = Provider`. Left in place they would bind the
        // registration impl's `Self` — the path key `PathCons<..>` — so a
        // dependency like `Self: HasErrorType` would demand `PathCons<..>:
        // HasErrorType` and never resolve. The impl carries only the parameters
        // that name the key and provider, plus the `__Components__` table.
        generics.where_clause = None;
        generics.params.push(parse_internal!(__Components__));

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #namespace_trait_path for #key_type
            #where_clause
            {
                type Delegate = #provider_type;
            }
        };

        // The impl is built from quasi-quoted tokens, so its boundary carries the
        // macro `call_site` span. Re-span it onto the user-written key token so a
        // coherence conflict (`E0119`) between two default impls for the same key
        // is reported on that key rather than on the whole `#[cgp_impl]` attribute.
        // Only the boundary moves; the interior tokens — the provider type, a
        // per-entry generic, each synthesized reference — keep their spans, so the
        // user's tokens stay navigable in an IDE, mirroring
        // `EvaluatedDelegateEntry::respan_impl`.
        override_item_span(key_type.span(), &item_impl)
    }
}

impl Parse for DefaultImplAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_type = input.parse()?;
        let in_token = input.parse()?;
        let namespace = input.parse()?;

        Ok(Self {
            key_type,
            in_token,
            namespace,
        })
    }
}
