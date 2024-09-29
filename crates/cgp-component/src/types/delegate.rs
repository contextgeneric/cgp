use core::marker::PhantomData;

pub struct DelegateTo<Components>(pub PhantomData<Components>);
