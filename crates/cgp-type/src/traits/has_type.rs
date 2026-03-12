use cgp_component::*;
use cgp_macro::cgp_component;

#[cgp_component {
    provider: TypeProvider,
    derive_delegate: UseDelegate<Tag>,
}]
pub trait HasType<Tag> {
    type Type;
}

pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
