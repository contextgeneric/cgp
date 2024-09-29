use cgp_component::types::delegate::DelegateTo;
use cgp_component::{derive_component, DelegateComponent, HasComponents};

#[derive_component(TypeComponent, ProvideType<Context>)]
pub trait HasType<Tag> {
    type Type;
}

impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for DelegateTo<Components>
where
    Components: DelegateComponent<Tag>,
    Components::Delegate: ProvideType<Context, Tag, Type = Type>,
{
    type Type = Type;
}
