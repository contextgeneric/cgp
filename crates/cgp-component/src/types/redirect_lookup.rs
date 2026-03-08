use core::marker::PhantomData;

pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);
