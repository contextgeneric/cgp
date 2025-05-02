use quote::quote;

use crate::replace_with;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_replace_tokens_with_generics() {
    let source = quote! {
        [
            FooComponent,
            <A> BarComponent<A>,
            <'b, B> BazComponent<'b, B>,
        ],
        [
            BarComponent,
        ],
        | Name | {
            delegate_components! {
                MyComponents {
                    Name: ParentComponents,
                }
            }
        }
    };

    let expected = quote! {
        delegate_components! {
            MyComponents {
                [
                    FooComponent,
                    <'b, B> BazComponent<'b, B>
                ]: ParentComponents,
            }
        }
    };

    let derived = replace_with(source).unwrap();

    assert!(equal_token_stream(&derived, &expected));
}
