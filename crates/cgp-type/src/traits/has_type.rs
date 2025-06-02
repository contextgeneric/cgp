use cgp_component::{DelegateComponent, HasCgpProvider, IsProviderFor, UseContext, UseDelegate};
use cgp_macro::cgp_component;

#[cgp_component {
    name: TypeComponent,
    provider: ProvideType,
    use_delegate: Tag,
}]
pub trait HasType<Tag> {
    type Type;
}

pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
