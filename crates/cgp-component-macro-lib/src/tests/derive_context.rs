use quote::quote;

use crate::derive_context;
use crate::tests::helper::equal::assert_equal_token_stream;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_derive_context() {
    let derived = derive_context(
        quote! { FooComponents },
        quote! {
            pub struct FooContext<Bar: BarConstraint>
            where
                Bar: BazConstraint,
            {
                pub bar: PhantomData<Bar>,
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct FooContext<Bar: BarConstraint>
        where
            Bar: BazConstraint,
        {
            pub bar: PhantomData<Bar>,
        }

        pub struct FooComponents;

        impl<Bar: BarConstraint> HasProvider for FooContext<Bar>
        where
            Bar: BazConstraint,
        {
            type Provider = FooComponents;
        }
    };

    println!("derived: {}", format_token_stream(&derived));

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_context_with_preset() {
    let derived = derive_context(
        quote! { FooComponents: FooPreset<BaseComponents> },
        quote! {
            pub struct FooContext<Bar: BarConstraint>
            where
                Bar: BazConstraint,
            {
                pub bar: PhantomData<Bar>,
            }
        },
    )
    .unwrap();

    let expected = quote! {
        pub struct FooContext<Bar: BarConstraint>
        where
            Bar: BazConstraint,
        {
            pub bar: PhantomData<Bar>,
        }

        pub struct FooComponents;

        impl<Bar: BarConstraint> HasProvider for FooContext<Bar>
        where
            Bar: BazConstraint,
        {
            type Provider = FooComponents;
        }

        impl<__Name__> DelegateComponent<__Name__> for FooComponents
        where
            Self: IsFooPreset<__Name__>,
        {
            type Delegate = FooPreset<BaseComponents>;
        }

        impl<__Name__, __Context__, __Params__> IsProviderFor<__Name__, __Context__, __Params__>
        for FooComponents
        where
            Self: IsFooPreset<__Name__>,
            FooPreset<BaseComponents>: IsProviderFor<__Name__, __Context__, __Params__>,
        {}
    };

    assert_equal_token_stream(&derived, &expected);
}
