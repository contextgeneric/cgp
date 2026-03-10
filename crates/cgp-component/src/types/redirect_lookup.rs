use core::marker::PhantomData;

use crate::{DelegateComponent, IsProviderFor};

pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);

pub trait HasDelegate<Components> {
    type Delegate;
}

impl<Path, Components> HasDelegate<Components> for Path
where
    Components: DelegateComponent<Path>,
{
    type Delegate = Components::Delegate;
}

// impl<Path, Component, Components> DelegateComponent<Component> for RedirectLookup<Path, Components>
// where
//     Components: DelegateComponent<Path>,
// {
//     type Delegate = Components::Delegate;
// }

// impl<Path, Component, Components, Context, Params> IsProviderFor<Component, Context, Params>
//     for RedirectLookup<Path, Components>
// where
//     Components: DelegateComponent<Path>,
//     Components::Delegate: IsProviderFor<Component, Context, Params>,
// {
// }

// delegate_components! {
//     <Path, Components: DelegateComponent<Path>, Component>
//     RedirectLookup<Path, Components> {
//         Component:
//             Components::Delegate,
//     }
// }
