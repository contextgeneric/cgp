use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Error, Fields, Ident, ItemImpl, ItemStruct};

pub fn derive_to_fields_ref_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let mut constructor = quote! { Nil };

    match &item_struct.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                constructor = quote! {
                    Cons(
                        (&self . #field_name) .into(),
                        #constructor
                    )
                };
            }
        }
        Fields::Unnamed(fields) => {
            let mut constructor = quote! { Nil };

            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_name: Ident = Ident::new(&format!("{i}"), field.span());

                constructor = quote! {
                    Cons(
                        (&self . #field_name) .into(),
                        #constructor
                    )
                };
            }
        }
        Fields::Unit => {}
    };

    let item_impl = parse2(quote! {
        impl #impl_generics
            ToFieldsRef for #struct_name #type_generics
        #where_clause
        {
            fn to_fields_ref<'a>(
                &'a self,
            ) -> Self::FieldsRef<'a>
            where
                Self: 'a,
            {
                #constructor
            }
        }
    })?;

    Ok(item_impl)
}
