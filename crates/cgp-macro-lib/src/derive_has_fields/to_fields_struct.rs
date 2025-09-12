use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse2, Error, Fields, Ident, ItemImpl, ItemStruct, LitInt};

pub fn derive_to_fields_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let constructor =
        derive_to_fields_constructor(&item_struct.fields, |field_name| match field_name {
            FieldLabel::None => quote! {
                self.0
            },
            _ => quote! {
                self #field_name .into()
            },
        })?;

    let item_impl = parse2(quote! {
        impl #impl_generics
            ToFields for #struct_name #type_generics
        #where_clause
        {
            fn to_fields(
                self,
            ) -> Self::Fields {
                #constructor
            }
        }
    })?;

    Ok(item_impl)
}

pub enum FieldLabel {
    Named(Ident),
    Unnamed(LitInt),
    None,
}

impl ToTokens for FieldLabel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Named(label) => tokens.extend(quote! { . #label }),
            Self::Unnamed(label) => tokens.extend(quote! { . #label }),
            Self::None => {}
        }
    }
}

pub fn derive_to_fields_constructor(
    fields: &Fields,
    construct_field: impl Fn(FieldLabel) -> TokenStream,
) -> syn::Result<TokenStream> {
    let mut constructors = quote! { ε };

    match &fields {
        Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().cloned().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                let constructor = construct_field(FieldLabel::Named(field_name));

                constructors = quote! {
                    π(
                        #constructor,
                        #constructors
                    )
                };
            }
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                // constructors = quote! {
                //     field
                // }
                constructors = construct_field(FieldLabel::None);
            } else {
                for (i, field) in fields.unnamed.iter().enumerate().rev() {
                    let field_name = LitInt::new(&format!("{i}"), field.span());

                    let constructor = construct_field(FieldLabel::Unnamed(field_name));

                    constructors = quote! {
                        π(
                            #constructor,
                            #constructors
                        )
                    };
                }
            }
        }
        Fields::Unit => {}
    };

    Ok(constructors)
}
