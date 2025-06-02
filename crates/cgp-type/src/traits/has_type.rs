use cgp_component::{DelegateComponent, HasCgpProvider, IsProviderFor, UseContext, UseDelegate};
use cgp_macro::cgp_component;

#[cgp_component {
    name: TypeComponent,
    provider: ProvideType,
    derive_delegate: UseDelegate<Tag>,
}]
pub trait HasType<Tag> {
    type Type;
}

pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
