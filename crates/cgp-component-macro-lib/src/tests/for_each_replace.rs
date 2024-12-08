use quote::quote;

use crate::for_each_replace::{handle_for_each_replace, handle_replace};
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_for_each_replace() {
    let source = quote! {
        [
            FooComponent,
            BarComponent,
            BazComponent,
        ],
        [
            BarComponent,
        ],
        | Name | {
            impl DelegateComponent<Name> for MyComponents {
                type Delegate = ParentComponents;
            }
        }
    };

    let expected = quote! {
        impl DelegateComponent<FooComponent> for MyComponents {
            type Delegate = ParentComponents;
        }

        impl DelegateComponent<BazComponent> for MyComponents {
            type Delegate = ParentComponents;
        }
    };

    let derived = handle_for_each_replace(source).unwrap();

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_for_each_replace_without_exclude() {
    let source = quote! {
        [
            FooComponent,
            BarComponent,
        ],
        | Name | {
            impl DelegateComponent<Name> for MyComponents {
                type Delegate = ParentComponents;
            }
        }
    };

    let expected = quote! {
        impl DelegateComponent<FooComponent> for MyComponents {
            type Delegate = ParentComponents;
        }

        impl DelegateComponent<BarComponent> for MyComponents {
            type Delegate = ParentComponents;
        }
    };

    let derived = handle_for_each_replace(source).unwrap();

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_for_each_replace_with_generics() {
    let source = quote! {
        [
            FooComponent,
            <A> BarComponent<A>,
            <'b, B> BazComponent<'b, B>,
        ],
        [
            BarComponent<A>,
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
                FooComponent: ParentComponents,
            }
        }

        delegate_components! {
            MyComponents {
                <'b, B> BazComponent<'b, B>: ParentComponents,
            }
        }
    };

    let derived = handle_for_each_replace(source).unwrap();

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_replace_tokens_with_generics() {
    let source = quote! {
        [
            FooComponent,
            <A> BarComponent<A>,
            <'b, B> BazComponent<'b, B>,
        ],
        [
            BarComponent<A>,
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

    let derived = handle_replace(source).unwrap();

    assert!(equal_token_stream(&derived, &expected));
}
