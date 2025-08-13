use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse2, FnArg, Ident, ItemTrait, TraitItemFn};

pub fn cgp_dispatch(_attr: TokenStream, mut out: TokenStream) -> syn::Result<TokenStream> {
    let item_trait: ItemTrait = parse2(out.clone())?;

    for item in item_trait.items.iter() {
        match item {
            syn::TraitItem::Fn(fn_item) => {
                let method_computer = derive_method_computer(&item_trait, fn_item)?;
                out.extend(method_computer);
            }
            _ => {
                return Err(syn::Error::new(
                    item.span(),
                    "Only function items are allowed in a dispatch trait",
                ));
            }
        }
    }

    Ok(out)
}

fn derive_method_computer(
    item_trait: &ItemTrait,
    method: &TraitItemFn,
) -> syn::Result<TokenStream> {
    let signature = &method.sig;
    let method_ident = &signature.ident;
    let return_type = &signature.output;
    let async_token = signature.asyncness;

    let generics = {
        let mut generics = item_trait.generics.clone();

        generics
            .params
            .extend(signature.generics.params.iter().cloned());
        if let Some(method_where_clause) = &signature.generics.where_clause {
            generics
                .make_where_clause()
                .predicates
                .extend(method_where_clause.predicates.iter().cloned());
        }

        let impl_generics = generics.split_for_impl().0;
        let trait_ident = &item_trait.ident;
        generics.params.insert(
            0,
            parse2(quote! {
                __Variants__: #trait_ident #impl_generics
            })?,
        );

        generics
    };

    let mut args = signature.inputs.iter();

    let receiver = if let Some(FnArg::Receiver(receiver)) = args.next() {
        receiver
    } else {
        return Err(syn::Error::new(
            signature.span(),
            "Dispatcher method must have a self argument",
        ));
    };

    let context_type = match (&receiver.reference, &receiver.mutability) {
        (Some((_, life)), Some(_)) => quote! { &mut #life __Variants__ },
        (Some((_, life)), None) => quote! { & #life __Variants__ },
        _ => quote! { __Variants__ },
    };

    let mut arg_idents = Punctuated::<_, Comma>::new();
    let mut arg_types = Punctuated::<_, Comma>::new();

    for (i, arg) in args.enumerate() {
        if let FnArg::Typed(pat_type) = arg {
            arg_idents.push(Ident::new(&format!("arg_{}", i), pat_type.span()));
            arg_types.push(pat_type.ty.as_ref().clone());
        } else {
            return Err(syn::Error::new(
                arg.span(),
                "Dispatcher method arguments must be typed",
            ));
        }
    }

    let arg_params = if arg_idents.is_empty() {
        TokenStream::new()
    } else {
        quote! {
            (#arg_idents): (#arg_types)
        }
    };

    let dot_await = if async_token.is_some() {
        quote! { .await }
    } else {
        TokenStream::new()
    };

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[cgp_computer]
        #async_token fn #method_ident #impl_generics (
            __Variants__: #context_type,
            #arg_params
        ) #return_type
        #where_clause
        {
            __Variants__. #method_ident( #arg_idents ) #dot_await
        }
    })
}
