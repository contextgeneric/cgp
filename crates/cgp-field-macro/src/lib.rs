extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(HasField)]
pub fn derive_fields(item: TokenStream) -> TokenStream {
    cgp_field_macro_lib::derive_fields(item.into()).into()
}

#[proc_macro]
pub fn symbol(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_symbol(body.into()).into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Product(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_product_type(body.into()).into()
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Sum(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_sum_type(body.into()).into()
}

#[proc_macro]
pub fn product(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_product_expr(body.into()).into()
}
