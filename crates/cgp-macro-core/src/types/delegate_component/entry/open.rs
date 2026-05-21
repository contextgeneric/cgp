use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use syn::{Ident, Type};

pub struct OpenDelegateEntry {
    pub open: Ident,
    pub components: Punctuated<Type, Comma>,
    pub semi: Semi,
}
