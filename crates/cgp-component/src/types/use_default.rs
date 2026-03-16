use core::marker::PhantomData;

use crate::DelegateComponent;

pub struct UseDefault;

pub trait DefaultComponentsNamespace<Components> {
    type Provider;
}

impl<Component> DelegateComponent<Component> for UseDefault
where
    Component: DefaultComponentsNamespace<UseDefault>,
{
    type Delegate = Component::Provider;
}

pub struct DefaultComponents<Component>(pub PhantomData<Component>);
