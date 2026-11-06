use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::{Error, Ident, ItemImpl, ItemStruct, ItemTrait, Type, braced};

use crate::functions::parse_internal;
use crate::traits::ParseOptionalKeyword;
use crate::types::delegate_component::{
    DelegateEntries, EvalDelegateEntries, EvalDelegateEntry, EvalForEntry,
    ExtractInnerDelegateTables,
};
use crate::types::generics::ImplGenerics;
use crate::types::ident::{IdentWithTypeArgs, PathWithTypeArgs};
use crate::types::keyword::Keyword;
use crate::types::keywords::New;
use crate::types::namespace::{EvaluatedNamespaceTable, InheritNamespaceStatement};

pub struct NamespaceTable {
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub namespace: IdentWithTypeArgs,
    pub parent_namespace: Option<(Colon, PathWithTypeArgs)>,
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
            reject_self_inheritance(&namespace_type, &parent_namespace_type)?;
            Some((colon, parent_namespace_type))
        } else {
            None
        };

        let entries = if input.is_empty() {
            Default::default()
        } else {
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

/// Reject a namespace that names itself as its parent, `new Ns: Ns` or `new Ns: self::Ns`, a cycle
/// visible in this one invocation. The parent's arguments do not matter: every instance of the
/// namespace has the same parent rule, so `<T> new Ns<T>: Ns<u32>` asks `Ns<u32>` again for a key
/// its table lacks, and `Ns<Vec<T>>` asks an ever larger instance. Any other qualified parent, such
/// as `crate::Ns` or `super::Ns`, is not checked, since the macro cannot tell which item the path
/// resolves to; the compiler then rejects a cycle with an unconstrained `__Value__`. A cycle across
/// several namespaces is not visible here.
fn reject_self_inheritance(
    namespace: &IdentWithTypeArgs,
    parent: &PathWithTypeArgs,
) -> syn::Result<()> {
    let segments = &parent.path.segments;
    let name = match segments.len() {
        1 => Some(&segments[0]),
        2 if segments[0].ident == "self" => Some(&segments[1]),
        _ => None,
    };
    let names_itself = parent.path.leading_colon.is_none()
        && name.is_some_and(|segment| segment.ident == namespace.ident);

    if names_itself {
        // On the path alone: the parent's `<…>` is rebuilt from its arguments with the macro's
        // span, which would widen the caret to the whole invocation.
        return Err(Error::new_spanned(
            &parent.path,
            format!(
                "the namespace `{}` cannot inherit from itself; name another namespace as its parent, or omit the parent",
                namespace.ident
            ),
        ));
    }

    Ok(())
}

impl NamespaceTable {
    pub fn build_namespace_trait(&self) -> syn::Result<Type> {
        let mut namespace = self.namespace.clone();

        namespace.type_args.args.push(parse_internal!(__Table__));

        let namespace_trait: Type = parse_internal!( #namespace );
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

    pub fn build_namespace_struct(&self) -> syn::Result<Option<ItemStruct>> {
        if self.new.is_none() {
            return Ok(None);
        }

        let namespace_ident = &self.namespace.ident;

        let namespace_struct_ident = Ident::new(
            &format!("__{}Components", namespace_ident),
            namespace_ident.span(),
        );

        let namespace_struct: ItemStruct = parse_internal! {
            pub struct #namespace_struct_ident;
        };

        Ok(Some(namespace_struct))
    }

    pub fn build_parent_namespace_impl(&self) -> syn::Result<Option<ItemImpl>> {
        let Some((_, parent_namespace)) = &self.parent_namespace else {
            return Ok(None);
        };

        let namespace_ident = self.namespace.ident.clone();

        let table_type: Type = parse_internal!(__Table__);

        let namespace_struct_ident = Ident::new(
            &format!("__{}Components", namespace_ident),
            namespace_ident.span(),
        );

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

        Ok(Some(item_impl))
    }

    pub fn eval(&self) -> syn::Result<EvaluatedNamespaceTable> {
        let item_trait = self.build_item_trait()?;
        let mut item_impls = self.build_item_impls()?;
        let item_struct = self.build_namespace_struct()?;

        if let Some(item_impl) = self.build_parent_namespace_impl()? {
            item_impls.insert(0, item_impl);
        }

        // Lift out each nested `Wrapper<new Inner { … }>` value, exactly as
        // `DelegateTable::eval` does. The entry's `Delegate` resolves to
        // `Wrapper<Inner>`, so without emitting `Inner` and its own
        // `DelegateComponent` impls the entry would name a type nothing declares.
        let mut inner_structs = Vec::new();

        for inner_table in self.entries.extract_inner_tables() {
            inner_structs.push(inner_table.build_table_struct());

            item_impls.extend(inner_table.build_impls()?);
        }

        Ok(EvaluatedNamespaceTable {
            item_impls,
            item_trait,
            item_struct,
            inner_structs,
        })
    }
}
