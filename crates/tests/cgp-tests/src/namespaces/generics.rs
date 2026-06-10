use core::fmt::Display;

use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;

#[cgp_component(ShowImpl)]
#[prefix(@test)]
pub trait Show<T> {
    fn show(&self, value: &T) -> String;
}

#[cgp_impl(new ShowWithDisplay)]
// #[default_impl(String in DefaultNamespace1<ShowImplComponent>)]
impl<T: Display> ShowImpl<T> {
    fn show(&self, value: &T) -> String {
        value.to_string()
    }
}

#[cgp_impl(new ShowWithString)]
#[default_impl(String in DefaultNamespace1<ShowImplComponent>)]
impl ShowImpl<String> {
    fn show(&self, value: &String) -> String {
        value.clone()
    }
}

// cgp_namespace! {
//     DefaultShow {
//         <T: Display> T:
//             @ShowWithDisplay,
//     }
// }

/*
    cgp_namespace! {
        DefaultNamespace1<ShowImplComponent> {
            String:
                ShowWithDisplay,
        }
    }
*/

impl<Components> DefaultImpls1<ShowImplComponent, Components> for String {
    type Delegate = ShowWithString;
}
