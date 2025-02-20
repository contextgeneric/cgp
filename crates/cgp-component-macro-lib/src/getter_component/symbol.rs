use syn::{Type, parse_quote};

pub fn symbol_from_string(value: &str) -> Type {
    value
        .chars()
        .rfold(parse_quote! { Nil }, |tail, c: char| -> Type {
            parse_quote!( Cons< Char< #c >, #tail > )
        })
}
