use core::marker::PhantomData;

use crate::DelegateComponent;

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
