use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::parse_file;

pub fn equal_token_stream(left: &TokenStream, right: &TokenStream) -> bool {
    format_token_stream(left) == format_token_stream(right)
}

pub fn format_token_stream(stream: &TokenStream) -> String {
    unparse(&parse_file(&stream.to_string()).unwrap())
}
