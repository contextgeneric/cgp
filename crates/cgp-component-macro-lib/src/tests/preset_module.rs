use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_preset_module;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_preset_module() {
    let derived = derive_preset_module(
        TokenStream::new(),
        quote! {
            mod preset {
                use cgp::prelude::*;
                use foo::{FooComponent, FooProvider};
                use bar::{BarComponent, BarProvider};

                cgp_preset! {
                    MyPreset {
                        FooComponent: FooProvider,
                        BarComponent: BarProvider,
                    }
                }
            }
        },
    )
    .unwrap();

    println!("derived: {}", format_token_stream(&derived));
}
