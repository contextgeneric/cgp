use proc_macro2::Span;
use syn::Ident;

pub fn to_snake_case(val: &Ident) -> Ident {
    let mut acc = String::new();
    let mut prev = '_';
    for ch in val.to_string().chars() {
        if ch.is_uppercase() && prev != '_' {
            acc.push('_');
        }
        acc.push(ch);
        prev = ch;
    }

    let raw_res = acc.to_lowercase();

    Ident::new(&raw_res, Span::call_site())
}
