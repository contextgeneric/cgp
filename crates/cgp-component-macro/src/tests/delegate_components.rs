use quote::quote;

use crate::delegate_components::delegate_components;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_delegate_components() {
    let derived = delegate_components(quote! {
        FooComponents {
            [
                BarAComponent,
                BarBComponent,
            ]: BazAComponents,
            BarCComponent: BazBComponents,
        }
    });

    println!("derived:\n{}", format_token_stream(&derived));
}

#[test]
fn test_delegate_components_containing_generics() {
    let derived = delegate_components(quote! {
        <'a, FooParam: FooConstraint>
        FooComponents<'a, FooParam> {
            [
                BarAComponent<'a>,
                BarBComponent<FooParam>,
            ]: BazAComponents,
            BarCComponent: BazBComponents<FooParam>,
        }
    });

    println!("derived:\n{}", format_token_stream(&derived));
}
