use cgp_component::{derive_component, DelegateComponent, DelegateTo, HasComponents, UseContext};

#[derive_component(TypeComponent, ProvideType<Context>)]
pub trait HasType<Tag> {
    type Type;
}

impl<Context, Tag> ProvideType<Context, Tag> for UseContext
where
    Context: HasType<Tag>,
{
    type Type = Context::Type;
}

impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for DelegateTo<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
