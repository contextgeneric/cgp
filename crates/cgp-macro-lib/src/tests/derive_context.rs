use quote::quote;

use crate::cgp_context;
use crate::tests::helper::equal::assert_equal_token_stream;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_derive_context() {
    let derived = cgp_context(
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

        impl<Bar: BarConstraint> HasCgpProvider for FooContext<Bar>
        where
            Bar: BazConstraint,
        {
            type CgpProvider = FooComponents;
        }
    };

    println!("derived: {}", format_token_stream(&derived));

    assert_equal_token_stream(&derived, &expected);
}

#[test]
fn test_derive_context_with_preset() {
    let derived = cgp_context(
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

        impl<Bar: BarConstraint> HasCgpProvider for FooContext<Bar>
        where
            Bar: BazConstraint,
        {
            type CgpProvider = FooComponents;
        }

        impl<__Name__> DelegateComponent<__Name__> for FooComponents
        where
            Self: FooPreset::IsPreset<__Name__>,
        {
            type Delegate = FooPreset::Provider<BaseComponents>;
        }

        impl<__Name__, __Context__, __Params__> IsProviderFor<__Name__, __Context__, __Params__>
        for FooComponents
        where
            Self: FooPreset::IsPreset<__Name__>,
            FooPreset::Provider<
                BaseComponents,
            >: IsProviderFor<__Name__, __Context__, __Params__>,
        {}
    };

    assert_equal_token_stream(&derived, &expected);
}
