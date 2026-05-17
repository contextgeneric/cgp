use core::marker::PhantomData;

pub struct WithProvider<Provider>(pub PhantomData<Provider>);
