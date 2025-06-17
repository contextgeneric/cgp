use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, GenericParam, Ident, ItemStruct, Type, TypeParam};

pub fn derive_builder(body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;

    let builder_struct = derive_builder_struct(&context_struct)?;

    Ok(quote! {
        #builder_struct
    })
}

pub fn derive_builder_struct(context_struct: &ItemStruct) -> syn::Result<ItemStruct> {
    let context_name = &context_struct.ident;

    let mut builder_struct = context_struct.clone();

    let builder_name = Ident::new(&format!("Partial{}", context_name), context_name.span());
    builder_struct.ident = builder_name;

    let generics = &mut builder_struct.generics;

    for (i, field) in builder_struct.fields.iter_mut().enumerate() {
        let generic_param_name = Ident::new(&format!("__F{}__", i), Span::call_site());

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = &field.ty;

        let mapped_type: Type = parse2(quote! {
            <#generic_param_name as MapType>::Mapped<#field_type>
        })?;

        field.ty = mapped_type;
    }

    Ok(builder_struct)
}
