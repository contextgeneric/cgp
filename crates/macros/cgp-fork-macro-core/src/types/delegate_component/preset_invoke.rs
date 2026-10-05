use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident, Path, Type};

use crate::exports::{DelegateComponent, IsProviderFor};

/// Emit `Preset::with_components!` so one preset entry becomes a
/// `DelegateComponent` / `IsProviderFor` pair per component. `exclude` drops
/// keys the surrounding table already implements, which is how a later entry
/// overrides a preset.
pub fn invoke_with_components(
    preset_module: &Path,
    table_type: &Type,
    generics: &Generics,
    exclude: &[Type],
) -> TokenStream {
    let (impl_generics, _, _) = generics.split_for_impl();

    let mut check_generics = generics.clone();
    check_generics.params.push(syn::parse_quote!(__Context__));
    check_generics
        .params
        .push(syn::parse_quote!(__Params__: ?Sized));
    let (check_impl_generics, _, _) = check_generics.split_for_impl();

    let existing_predicates = generics
        .where_clause
        .as_ref()
        .map(|clause| &clause.predicates);

    quote! {
        #preset_module::with_components! {
            exclude [ #(#exclude),* ],
            __Component__ => {
                impl #impl_generics #DelegateComponent<__Component__> for #table_type
                where
                    #existing_predicates
                    #preset_module::Provider: #DelegateComponent<__Component__>,
                {
                    type Delegate = <#preset_module::Provider as #DelegateComponent<__Component__>>::Delegate;
                }

                impl #check_impl_generics #IsProviderFor<__Component__, __Context__, __Params__>
                    for #table_type
                where
                    #existing_predicates
                    #preset_module::Provider: #DelegateComponent<__Component__>,
                    <#preset_module::Provider as #DelegateComponent<__Component__>>::Delegate:
                        #IsProviderFor<__Component__, __Context__, __Params__>,
                {}
            }
        }
    }
}

/// The `with_components!` macro a preset module exports. It repeats `body` for
/// each of the preset's own keys, then forwards to each parent preset.
///
/// `#[macro_export]` places `#macro_name` at the crate root so `pub use` can
/// re-export it. A plain `macro_rules` cannot be `pub`.
pub fn define_with_components_macro(
    macro_name: &Ident,
    keys: &[Type],
    parents: &[Path],
) -> TokenStream {
    quote! {
        #[macro_export]
        #[doc(hidden)]
        macro_rules! #macro_name {
            ( $ph:ident => { $($body:tt)* } ) => {
                #macro_name! { exclude [], $ph => { $($body)* } }
            };
            ( exclude [ $($exclude:ty),* $(,)? ], $ph:ident => { $($body:tt)* } ) => {
                ::cgp_fork::prelude::cgp_for_each! {
                    [ #(#keys),* ],
                    exclude [ $($exclude),* ],
                    $ph => { $($body)* }
                }
                #(
                    #parents::with_components! {
                        exclude [ $($exclude),* ],
                        $ph => { $($body)* }
                    }
                )*
            };
        }
    }
}
