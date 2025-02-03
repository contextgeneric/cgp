use cgp_component::{DelegateComponent, HasComponents, IsProviderFor, UseContext, UseDelegate};
use cgp_component_macro::{cgp_component, cgp_provider};

#[cgp_component {
    name: TypeComponent,
    provider: ProvideType,
}]
pub trait HasType<Tag> {
    type Type;
}

pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;

#[cgp_provider(TypeComponent)]
impl<Context, Tag> ProvideType<Context, Tag> for UseContext
where
    Context: HasType<Tag>,
{
    type Type = Context::Type;
}

#[cgp_provider(TypeComponent)]
impl<Context, Tag, Components, Delegate> ProvideType<Context, Tag> for UseDelegate<Components>
where
    Components: DelegateComponent<Tag, Delegate = Delegate>,
    Delegate: ProvideType<Context, Tag>,
{
    type Type = Delegate::Type;
}
