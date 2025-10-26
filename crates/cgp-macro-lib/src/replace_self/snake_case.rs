use alloc::string::{String, ToString};

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
    let str_val = val.to_string();
    let mut snake_case_val = to_snake_case_str(&str_val);
    if !str_val.starts_with('_') {
        snake_case_val = format!("__{}__", snake_case_val);
    }

    Ident::new(&snake_case_val, Span::call_site())
}
