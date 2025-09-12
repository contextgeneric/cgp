use quote::{quote, ToTokens};
use syn::parse_quote;

use crate::derive_provider::derive_is_provider_for;
use crate::tests::helper::equal::equal_token_stream;

#[test]
fn test_derive_is_provider_for() -> syn::Result<()> {
    let derived = derive_is_provider_for(
        &parse_quote!(FooComponent),
        &parse_quote! {
            impl<Context, Bar: Async> FooProvider<Context, Bar, Baz> for UseFoo
            where
                Context: HasFooType,
                Bar: HasBarType,
            {
                fn foo() -> &str {
                    "foo"
                }
            }
        },
    )?
    .to_token_stream();

    let expected = quote! {
        impl<Context, Bar: Async> IsProviderFor<FooComponent, Context, (Bar, Baz)> for UseFoo
        where
                Context: HasFooType,
                Bar: HasBarType,
        {}
    };

    assert!(equal_token_stream(&derived, &expected));

    Ok(())
}
