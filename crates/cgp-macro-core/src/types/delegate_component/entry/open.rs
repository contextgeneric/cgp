use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};
use syn::{Ident, Type};

use crate::define_keyword;

define_keyword!(Open, OpenKeyword, "open");

pub struct OpenDelegateEntry {
    pub open: Ident,
    pub components: Punctuated<Type, Comma>,
    pub semi: Semi,
}
