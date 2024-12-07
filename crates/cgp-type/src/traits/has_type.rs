use cgp_component::{cgp_component, DelegateComponent, HasComponents, UseContext, UseDelegate};

#[cgp_component {
    name: TypeComponent,
    provider: ProvideType,
}]
pub trait HasType<Tag> {
    type Type;
}

impl<Context, Tag> ProvideType<Context, Tag> for UseContext
where
    Context: HasType<Tag>,
{
    type Type = Context::Type;
}

impl<Context, Tag, Components, Type> ProvideType<Context, Tag> for UseDelegate<Components>
where
    Components: DelegateComponent<Tag>,
    Components::Delegate: ProvideType<Context, Tag, Type = Type>,
{
    type Type = Type;
}
