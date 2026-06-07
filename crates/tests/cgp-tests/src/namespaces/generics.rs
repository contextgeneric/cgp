use core::fmt::Display;

use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;

#[cgp_component(ShowImpl)]
#[prefix(@test)]
pub trait Show<T> {
    fn show(&self, value: &T) -> String;
}

#[cgp_impl(new ShowWithDisplay)]
// #[default_impl(DefaultNamespace1)]
impl<T: Display> ShowImpl<T> {
    fn show(&self, value: &T) -> String {
        value.to_string()
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
    type Delegate = ShowWithDisplay;
}
