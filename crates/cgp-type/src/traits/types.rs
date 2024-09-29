use cgp_component::{derive_component, DelegateComponent, HasComponents};

#[derive_component(TypeComponent, ProvideType<Context>)]
pub trait HasType<Tag> {
    type Type;
}
