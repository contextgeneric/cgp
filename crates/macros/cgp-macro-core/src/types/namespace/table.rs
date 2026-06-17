use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::{Error, Ident, ItemImpl, ItemStruct, ItemTrait, Type, braced};

use crate::parse_internal;
use crate::traits::ParseOptionalKeyword;
use crate::types::delegate_component::{
    DelegateEntries, EvalDelegateEntries, EvalDelegateEntry, EvalForEntry,
};
use crate::types::generics::ImplGenerics;
use crate::types::ident::{IdentWithTypeArgs, IdentWithTypeGenerics};
use crate::types::keyword::Keyword;
use crate::types::keywords::New;
use crate::types::namespace::{EvaluatedNamespaceTable, InheritNamespaceStatement};

pub struct NamespaceTable {
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub namespace: IdentWithTypeGenerics,
    pub parent_namespace: Option<(Colon, IdentWithTypeArgs)>,
    pub entries: DelegateEntries,
}

impl Parse for NamespaceTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = input.parse()?;

        let new = input.parse_optional_keyword()?;

        let namespace_type = input.parse()?;
        let parent_namespace = if input.peek(Colon) {
            let colon: Colon = input.parse()?;
            let parent_namespace_type = input.parse()?;
            Some((colon, parent_namespace_type))
        } else {
            None
        };

        let entries = {
            let body;
            braced!(body in input);

            body.parse()?
        };

        Ok(Self {
            impl_generics,
            new,
            namespace: namespace_type,
            parent_namespace,
            entries,
        })
    }
}

impl NamespaceTable {
    pub fn build_namespace_trait(&self) -> syn::Result<Type> {
        let namespace_ident = &self.namespace.ident;
        let mut namespace_generics = self.namespace.type_generics.clone();
        namespace_generics.params.push(parse_internal!(__Table__));

        let namespace_trait: Type = parse_internal!( #namespace_ident #namespace_generics );
        Ok(namespace_trait)
    }

    pub fn build_item_trait(&self) -> syn::Result<Option<ItemTrait>> {
        let namespace_trait = self.build_namespace_trait()?;

        let item_trait: Option<ItemTrait> = if self.new.is_some() {
            let item_trait = parse_internal! {
                pub trait #namespace_trait {
                    type Delegate;
                }
            };

            Some(item_trait)
        } else {
            None
        };

        Ok(item_trait)
    }

    pub fn build_item_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        let mut impl_generics = self.impl_generics.clone();
        impl_generics.params.push(parse_internal!(__Table__));

        let namespace_trait = self.build_namespace_trait()?;
        let table_type: Type = parse_internal!(__Table__);

        let evaluated_entries = self.entries.eval_entries(&table_type)?;

        let mut item_impls: Vec<ItemImpl> = Vec::new();

        for evaluated_entry in evaluated_entries {
            let item_impl =
                evaluated_entry.build_namespace_impl(&namespace_trait, &impl_generics)?;

            item_impls.push(item_impl);
        }

        Ok(item_impls)
    }

    pub fn build_parent_namespace_impl(&self) -> syn::Result<Option<(ItemStruct, ItemImpl)>> {
        let Some((_, parent_namespace)) = &self.parent_namespace else {
            return Ok(None);
        };

        if self.new.is_none() {
            return Err(Error::new(
                parent_namespace.ident.span(),
                "parent namespace can only be specified with `new` namespaces",
            ));
        }

        let namespace_ident = self.namespace.ident.clone();

        let table_type: Type = parse_internal!(__Table__);

        let namespace_struct_ident = Ident::new(
            &format!("__{}Components", namespace_ident),
            namespace_ident.span(),
        );

        let namespace_struct: ItemStruct = parse_internal! {
            pub struct #namespace_struct_ident;
        };

        let for_entry = InheritNamespaceStatement {
            namespace: parent_namespace.clone(),
            local_table_ident: namespace_struct_ident,
        }
        .eval_for_entry(&table_type)?;

        let evaluated_entry = for_entry.eval_entry(&table_type)?;

        let namespace_trait = self.build_namespace_trait()?;

        let mut generics = self.impl_generics.generics.clone();
        generics.params.push(parse_internal!(#table_type));

        let item_impl = evaluated_entry.build_namespace_impl(&namespace_trait, &generics)?;

        Ok(Some((namespace_struct, item_impl)))
    }

    pub fn eval(&self) -> syn::Result<EvaluatedNamespaceTable> {
        let mut item_struct = None;
        let item_trait = self.build_item_trait()?;
        let mut item_impls = self.build_item_impls()?;

        if let Some((namespace_struct, item_impl)) = self.build_parent_namespace_impl()? {
            item_impls.insert(0, item_impl);
            item_struct = Some(namespace_struct);
        }

        Ok(EvaluatedNamespaceTable {
            item_impls,
            item_trait,
            item_struct,
        })
    }
}
