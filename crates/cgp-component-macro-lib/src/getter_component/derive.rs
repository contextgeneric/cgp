use alloc::string::ToString;
use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::token::Mut;
use syn::{parse_quote, Error, FnArg, Ident, ItemImpl, ItemTrait, ReturnType, TraitItem, Type};

use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::derive::derive_component_with_ast;
use crate::derive_component::replace_self_type::replace_self_type;

pub fn derive_getter_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(item)?;

    let derived_component = derive_component_with_ast(&spec, &consumer_trait)?;

    let fields = parse_getter_fields(&spec, &consumer_trait)?;

    let use_fields_impl = derive_use_fields_impl(&spec, &consumer_trait, &fields);

    let m_field: Option<[GetterField; 1]> = fields.try_into().ok();

    let mut derived = quote! {
        #derived_component

        #use_fields_impl
    };

    if let Some([field]) = m_field {
        let use_field_impl = derive_use_field_impl(&spec, &consumer_trait, &field);
        let use_provider_impl = derive_with_provider_impl(&spec, &consumer_trait, &field);

        derived.extend(use_field_impl);
        derived.extend(use_provider_impl);
    }

    Ok(derived)
}

pub struct GetterField {
    pub field_name: Ident,
    pub field_type: Type,
    pub provider_type: Type,
    pub field_mut: Option<Mut>,
}

pub fn derive_use_fields_impl(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
) -> ItemImpl {
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
    let mut constraints = consumer_trait.supertraits.clone();

    let mut methods: TokenStream = TokenStream::new();

    for field in fields {
        let field_name = &field.field_name;
        let provider_type = &field.provider_type;
        let field_symbol = symbol_from_string(&field.field_name.to_string());

        if field.field_mut.is_none() {
            constraints.push(parse_quote! {
                HasField< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( context: & #context_type ) -> & #provider_type {
                    context.get_field( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        } else {
            constraints.push(parse_quote! {
                HasFieldMut< #field_symbol, Value = #provider_type >
            });

            methods.extend(quote! {
                fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                    context.get_field_mut( ::core::marker::PhantomData::< #field_symbol > )
                }
            });
        }
    }

    parse_quote! {
        impl< #context_type > #provider_name < #context_type > for UseFields
        where
            #context_type: #constraints
        {
            #methods
        }
    }
}

pub fn derive_use_field_impl(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
    field: &GetterField,
) -> TokenStream {
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
    let mut constraints = consumer_trait.supertraits.clone();

    let field_name = &field.field_name;
    let provider_type = &field.provider_type;

    let method = if field.field_mut.is_none() {
        constraints.push(parse_quote! {
            HasField< Tag, Value = #provider_type >
        });

        quote! {
            fn #field_name( context: & #context_type ) -> & #provider_type {
                context.get_field( ::core::marker::PhantomData )
            }
        }
    } else {
        constraints.push(parse_quote! {
            HasFieldMut< Tag, Value = #provider_type >
        });

        quote! {
            fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                context.get_field_mut( ::core::marker::PhantomData )
            }
        }
    };

    let use_field_impl: ItemImpl = parse_quote! {
        impl< #context_type, Tag > #provider_name < #context_type > for UseField<Tag>
        where
            #context_type: #constraints
        {
            #method
        }
    };

    quote! {
        #use_field_impl
    }
}

pub fn derive_with_provider_impl(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
    field: &GetterField,
) -> TokenStream {
    let component_name = &spec.component_name;
    let context_type = &spec.context_type;
    let provider_name = &spec.provider_name;

    // FIXME: replace `Self` with `Context` inside super trait bound
    let context_constraints = consumer_trait.supertraits.clone();

    let field_name = &field.field_name;
    let provider_type = &field.provider_type;

    let provider_constraint = if field.field_mut.is_none() {
        quote! {
            FieldGetter< #context_type, #component_name, Value = #provider_type >
        }
    } else {
        quote! {
            MutFieldGetter< #context_type, #component_name, Value = #provider_type >
        }
    };

    let method = if field.field_mut.is_none() {
        quote! {
            fn #field_name( context: & #context_type ) -> & #provider_type {
                Provider::get_field(context, ::core::marker::PhantomData )
            }
        }
    } else {
        quote! {
            fn #field_name( context: &mut #context_type ) -> &mut #provider_type {
                Provider::get_field_mut(context, ::core::marker::PhantomData )
            }
        }
    };

    quote! {
        impl< #context_type, Provider > #provider_name < #context_type > for WithProvider<Provider>
        where
            #context_type: #context_constraints,
            Provider: #provider_constraint,
        {
            #method
        }
    }
}

pub fn parse_getter_fields(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
) -> syn::Result<Vec<GetterField>> {
    if !consumer_trait.generics.params.is_empty() {
        return Err(Error::new(
            consumer_trait.generics.params.span(),
            "getter trait cannot contain generic parameters",
        ));
    }

    let mut fields = Vec::new();

    for item in consumer_trait.items.iter() {
        match item {
            TraitItem::Fn(method) => {
                let signature = &method.sig;

                if signature.constness.is_some() {
                    return Err(Error::new(
                        signature.constness.span(),
                        "getter method must not be const fn",
                    ));
                }

                if signature.asyncness.is_some() {
                    return Err(Error::new(
                        signature.asyncness.span(),
                        "getter method must not be async fn",
                    ));
                }

                if signature.unsafety.is_some() {
                    return Err(Error::new(
                        signature.unsafety.span(),
                        "getter method must not be unsafe fn",
                    ));
                }

                if !signature.generics.params.is_empty() {
                    return Err(Error::new(
                        signature.generics.params.span(),
                        "getter method must not contain generic param",
                    ));
                }

                if signature.generics.where_clause.is_some() {
                    return Err(Error::new(
                        signature.generics.where_clause.span(),
                        "getter method must not contain where clause",
                    ));
                }

                let field_name = signature.ident.clone();

                let [arg]: [&FnArg; 1] = signature
                    .inputs
                    .iter()
                    .collect::<Vec<&FnArg>>()
                    .try_into()
                    .map_err(|_| {
                        Error::new(
                            signature.inputs.span(),
                            "getter method must contain exactly one `&self` argument",
                        )
                    })?;

                let field_mut = match arg {
                    FnArg::Receiver(receiver) => {
                        if receiver.reference.is_none() {
                            return Err(Error::new(
                                receiver.span(),
                                "first argument to getter method must be a reference to self, i.e. `&self`"
                            ));
                        }

                        receiver.mutability
                    }
                    _ => {
                        return Err(Error::new(
                            arg.span(),
                            "first argument to getter method must be `&self`",
                        ))
                    }
                };

                let field_type: Type = match &signature.output {
                    ReturnType::Default => parse_quote!(()),
                    ReturnType::Type(_, ty) => {
                        let ty = ty.as_ref().clone();
                        match &ty {
                            Type::Reference(type_ref) => {
                                if type_ref.mutability.is_some() != field_mut.is_some() {
                                    return Err(Error::new(
                                        type_ref.span(),
                                        "return type have the same mutability as the self reference",
                                    ));
                                }

                                type_ref.elem.as_ref().clone()
                            }
                            _ => {
                                return Err(Error::new(
                                    ty.span(),
                                    "return type must be a reference",
                                ))
                            }
                        }
                    }
                };

                let provider_type: Type = syn::parse2(replace_self_type(
                    field_type.to_token_stream(),
                    &spec.context_type,
                    &Vec::new(),
                ))?;

                fields.push(GetterField {
                    field_name,
                    field_type,
                    provider_type,
                    field_mut,
                })
            }
            _ => {
                return Err(Error::new(
                    item.span(),
                    "getter trait can only contain getter methods",
                ))
            }
        }
    }

    Ok(fields)
}

pub fn symbol_from_string(value: &str) -> Type {
    value
        .chars()
        .rfold(parse_quote! { Nil }, |tail, c: char| -> Type {
            parse_quote!( Cons< Char< #c >, #tail > )
        })
}
