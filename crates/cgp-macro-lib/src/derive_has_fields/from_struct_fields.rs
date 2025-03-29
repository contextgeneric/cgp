use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Error, Fields, Ident, ItemImpl, ItemStruct};

pub fn derive_from_fields_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let item_impl: ItemImpl = match &item_struct.fields {
        Fields::Named(fields) => {
            let mut fields_arg = quote! { Nil };
            let mut constructor_args = quote! {};

            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                fields_arg = quote! {
                    Cons( #field_name, #fields_arg )
                };

                constructor_args = quote! {
                    #field_name : #field_name .value ,
                    #constructor_args
                };
            }

            parse2(quote! {
                impl #impl_generics
                    FromFields for #struct_name #type_generics
                #where_clause
                {
                    fn from_fields(
                        #fields_arg: Self::Fields,
                    ) -> Self {
                        Self {
                            #constructor_args
                        }
                    }
                }
            })?
        }
        Fields::Unnamed(fields) => {
            let mut fields_arg = quote! { Nil };
            let mut constructor_args = quote! {};

            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_name: Ident = Ident::new(&format!("field_{i}"), field.span());

                fields_arg = quote! {
                    Cons( #field_name, #fields_arg )
                };

                constructor_args = quote! {
                    #field_name .value ,
                    #constructor_args
                };
            }

            parse2(quote! {
                impl #impl_generics
                    FromFields for #struct_name #type_generics
                #where_clause
                {
                    fn from_fields(
                        #fields_arg: Self::Fields,
                    ) -> Self {
                        Self (
                            #constructor_args
                        )
                    }
                }
            })?
        }
        Fields::Unit => parse2(quote! {
            impl #impl_generics
                FromFields for #struct_name #type_generics
            #where_clause
            {
                fn from_fields(
                    fields: Nil,
                ) -> Self {
                    Self
                }
            }
        })?,
    };

    Ok(item_impl)
}
