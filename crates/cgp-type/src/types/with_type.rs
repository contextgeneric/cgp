use core::marker::PhantomData;

pub struct WithTypeProvider<Provider>(pub PhantomData<Provider>);
