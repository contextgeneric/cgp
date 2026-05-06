use core::iter;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Bracket, Colon, Comma, Gt, Lt, RArrow, Semi};
use syn::{Error, Generics, Ident, Token, Type, braced, bracketed, parse_quote, parse2};

use crate::delegate_components::merge_generics;
use crate::parse::{ComponentPaths, ImplGenerics, SimpleType, TypeGenerics};

pub struct DelegateComponents {
    pub new_struct: bool,
    pub target_type: Type,
    pub target_generics: ImplGenerics,
    pub entries: Punctuated<DelegateEntry<Type>, Comma>,
}

#[derive(Clone)]
pub struct DelegateEntry<T> {
    pub keys: Punctuated<DelegateKey<T>, Comma>,
    pub mode: DelegateMode,
    pub value: DelegateValue,
}

#[derive(Clone)]
pub struct DelegateKey<T> {
    pub ty: T,
    pub generics: ImplGenerics,
}

#[derive(Clone)]
pub enum DelegateValue {
    Type(Type),
    New(DelegateNewValue),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum DelegateMode {
    Provider(Colon),
    Direct(RArrow),
}

#[derive(Clone)]
pub struct DelegateNewValue {
    pub wrapper_ident: Ident,
    pub struct_ident: Ident,
    pub struct_generics: Generics,
    pub entries: Punctuated<DelegateEntry<Type>, Comma>,
}

impl DelegateMode {
    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Direct(_))
    }
}
impl DelegateValue {
    pub fn as_type(&self) -> Type {
        match self {
            Self::Type(ty) => ty.clone(),
            Self::New(value) => {
                let wrapper_ident = &value.wrapper_ident;
                let struct_ident = &value.struct_ident;
                let (_, struct_generics, _) = value.struct_generics.split_for_impl();
                parse_quote!( #wrapper_ident < #struct_ident #struct_generics > )
            }
        }
    }
}

impl Parse for DelegateComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let new_struct = {
            let fork = input.fork();
            let new_ident: Option<Ident> = fork.parse().ok();
            match new_ident {
                Some(new_ident) if new_ident == "new" => {
                    input.advance_to(&fork);
                    true
                }
                _ => false,
            }
        };

        let target_type: Type = input.parse()?;

        let content;
        braced!(content in input);

        let meta_entries = parse_meta_delegate_entries(&content, &target_type)?;

        let delegate_entries: Punctuated<DelegateEntry<Type>, Comma> =
            Punctuated::parse_terminated(&content)?;

        let entries = meta_entries.into_iter().chain(delegate_entries).collect();

        Ok(Self {
            new_struct,
            target_type,
            target_generics,
            entries,
        })
    }
}

pub fn parse_meta_delegate_entries(
    input: ParseStream,
    target_type: &Type,
) -> syn::Result<Vec<DelegateEntry<Type>>> {
    let mut entries = Vec::new();

    while input.peek(Ident) {
        let fork = input.fork();
        let keyword: Ident = fork.parse()?;

        if keyword == "open" {
            input.advance_to(&fork);

            let components: Punctuated<Type, Comma> = Punctuated::parse_separated_nonempty(input)?;
            let _: Semi = input.parse()?;

            for component in components {
                let value = DelegateValue::Type(parse2(
                    quote!(RedirectLookup<#target_type, PathCons<#component, PathNil>>),
                )?);

                let key = DelegateKey {
                    ty: component,
                    generics: Default::default(),
                };

                let entry = DelegateEntry {
                    keys: Punctuated::from_iter([key]),
                    mode: DelegateMode::Provider(Colon(Span::call_site())),
                    value,
                };

                entries.push(entry)
            }
        } else if keyword == "namespace" {
            input.advance_to(&fork);

            let ident: Ident = input.parse()?;
            let _: Semi = input.parse()?;

            let namespace_ident = if ident == "default" {
                Ident::new("DefaultNamespace", ident.span())
            } else {
                ident
            };

            let delegate_key: Type = parse2(quote! {
                __Component__
            })?;

            let generics: ImplGenerics = parse2(quote! {
                <__Component__: #namespace_ident< #target_type >>
            })?;

            let delegate_value: Type = parse2(quote! {
                < __Component__ as #namespace_ident< #target_type >>::Provider
            })?;

            let entry = DelegateEntry {
                keys: Punctuated::from_iter([DelegateKey {
                    ty: delegate_key,
                    generics,
                }]),
                mode: DelegateMode::Provider(Colon(Span::call_site())),
                value: DelegateValue::Type(delegate_value),
            };

            entries.push(entry)
        } else {
            break;
        }
    }

    Ok(entries)
}

impl Parse for DelegateEntry<Type> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(DelegateKey::parse, Token![,])?
        } else {
            let impl_generics: Option<ImplGenerics> = if input.peek(Lt) {
                Some(input.parse()?)
            } else {
                None
            };

            if input.peek(At) {
                let _: At = input.parse()?;

                let path: ComponentPaths = input.parse()?;

                let mut keys = Punctuated::new();

                for path in path.paths {
                    let generics = if let Some(impl_generics) = &impl_generics {
                        ImplGenerics {
                            generics: merge_generics(
                                &impl_generics.generics,
                                &path.generics.generics,
                            ),
                        }
                    } else {
                        path.generics
                    };

                    let key = DelegateKey {
                        ty: path.path_type,
                        generics,
                    };

                    keys.push(key);
                }

                keys
            } else {
                let mut key: DelegateKey<Type> = input.parse()?;

                if let Some(impl_generics) = impl_generics {
                    key.generics = impl_generics;
                }

                Punctuated::from_iter(iter::once(key))
            }
        };

        let mode = input.parse()?;

        let source = input.parse()?;

        Ok(Self {
            keys: components,
            mode,
            value: source,
        })
    }
}

impl Parse for DelegateEntry<SimpleType> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(DelegateKey::parse, Token![,])?
        } else {
            let component: DelegateKey<SimpleType> = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let mode = input.parse()?;

        let source = input.parse()?;

        Ok(Self {
            keys: components,
            mode,
            value: source,
        })
    }
}

impl<Type: Parse> Parse for DelegateKey<Type> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_generics: ImplGenerics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let component_type: Type = input.parse()?;

        Ok(Self {
            ty: component_type,
            generics: component_generics,
        })
    }
}

impl Parse for DelegateMode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(RArrow) {
            Ok(Self::Direct(input.parse()?))
        } else {
            Ok(Self::Provider(input.parse()?))
        }
    }
}

impl Parse for DelegateValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if let Ok(value) = fork.parse::<DelegateNewValue>() {
            input.advance_to(&fork);
            return Ok(Self::New(value));
        }

        let ty: Type = input.parse()?;
        Ok(Self::Type(ty))
    }
}

impl Parse for DelegateNewValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper_ident = input.parse()?;

        let _: Lt = input.parse()?;

        let new_ident: Ident = input.parse()?;

        if new_ident != "new" {
            return Err(Error::new(new_ident.span(), "expect `new` keyword"));
        }

        let struct_ident = input.parse()?;

        let struct_generics: TypeGenerics = input.parse()?;

        let entries = {
            let content;
            braced!(content in input);

            Punctuated::parse_terminated(&content)?
        };

        let _: Gt = input.parse()?;

        Ok(Self {
            wrapper_ident,
            struct_ident,
            struct_generics: struct_generics.generics,
            entries,
        })
    }
}

impl<Type> ToTokens for DelegateEntry<Type>
where
    Type: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let components = &self.keys;
        let mode = &self.mode;
        let source = &self.value;

        let count = components.len();

        #[allow(clippy::comparison_chain)]
        if count == 1 {
            tokens.append_all(quote! {
                #components #mode #source
            });
        } else if count > 1 {
            tokens.append_all(quote! {
                [
                    #components
                ] #mode #source
            });
        }
    }
}

impl<Type> ToTokens for DelegateKey<Type>
where
    Type: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.generics.to_token_stream());
        tokens.extend(self.ty.to_token_stream());
    }
}

impl ToTokens for DelegateMode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Provider(colon) => colon.to_tokens(tokens),
            Self::Direct(arrow) => arrow.to_tokens(tokens),
        }
    }
}

impl ToTokens for DelegateValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(value) => value.to_tokens(tokens),
            Self::New(value) => value.to_tokens(tokens),
        }
    }
}

impl ToTokens for DelegateNewValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            wrapper_ident,
            struct_ident,
            struct_generics,
            entries,
        } = self;

        tokens.extend(quote! {
            #wrapper_ident <
                new #struct_ident #struct_generics {
                    #entries
                }
            >
        });
    }
}
