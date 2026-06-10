use core::fmt::Display;

use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;

#[cgp_component(ShowImpl)]
#[prefix(@test)]
pub trait Show<T> {
    fn show(&self, value: &T) -> String;
}

#[cgp_impl(new ShowWithString)]
#[default_impl(String in DefaultImpls1<ShowImplComponent>)]
impl ShowImpl<String> {
    fn show(&self, value: &String) -> String {
        value.clone()
    }
}

#[cgp_impl(new ShowWithDisplay)]
impl<T: Display> ShowImpl<T> {
    fn show(&self, value: &T) -> String {
        value.to_string()
    }
}

cgp_namespace! {
    new DefaultShowComponents {
        [
            String,
            u64,
        ]:
            ShowWithDisplay,
    }
}
