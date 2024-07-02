use syn::Ident;


pub fn define_delegates_to_trait(
    components_ident: &Ident,
    components: impl Iterator<Item = ComponentAst>,
) {
    let trait_bounds: Punctuated<TypeParamBound, Plus> = Default::default();
}