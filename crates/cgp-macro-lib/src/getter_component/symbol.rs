use syn::{parse_quote, Type};

pub fn symbol_from_string(value: &str) -> Type {
    value
        .chars()
        .rfold(parse_quote! { Nil }, |tail, c: char| -> Type {
            parse_quote!( Cons< Char< #c >, #tail > )
        })
}
