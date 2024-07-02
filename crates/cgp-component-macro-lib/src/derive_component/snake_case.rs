use proc_macro2::Span;
use syn::Ident;

pub fn to_snake_case_str(val: &str) -> String {
    let mut acc = String::new();
    let mut prev = '_';

    for ch in val.chars() {
        if ch.is_uppercase() && prev != '_' {
            acc.push('_');
        }
        acc.push(ch);
        prev = ch;
    }

    acc.to_lowercase()
}

pub fn to_snake_case_ident(val: &Ident) -> Ident {
    Ident::new(&to_snake_case_str(&val.to_string()), Span::call_site())
}
