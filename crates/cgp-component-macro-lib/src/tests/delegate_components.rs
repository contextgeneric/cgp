use quote::quote;

use crate::delegate_components::define_components;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_delegate_components() {
    let derived = define_components(quote! {
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
    let derived = define_components(quote! {
        FooComponents<'a, FooParamA, FooParamB: FooConstraint> {
            BarComponentA: BazComponentsA<FooParamA>,
            [
                BarComponentB<'a>,
                BarComponentC<FooParamB>,
                <BarParamA> BarComponentD<BarParamA, FooParamA>,
                <'b, BarParamB: BarConstraint> BarComponentE<BarParamB, FooParamB>,
            ]: BazComponentsB,
        }
    });

    println!("derived:\n{}", format_token_stream(&derived));
}
