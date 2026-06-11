use syn::Ident;

pub struct CgpComponentRawArgs {
    pub context: Option<Ident>,
    pub provider: Option<Ident>,
}
