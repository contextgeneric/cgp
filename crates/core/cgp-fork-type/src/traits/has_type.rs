use cgp_fork::macro_prelude::*;
use cgp_fork_macro::cgp_component;

#[cgp_component(TypeProvider)]
#[derive_delegate(UseDelegate<Tag>)]
pub trait HasType<Tag> {
    type Type;
}

pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
