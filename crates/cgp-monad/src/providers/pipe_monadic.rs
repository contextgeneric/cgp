use std::marker::PhantomData;

pub struct PipeMonadic<M, Providers>(pub PhantomData<(M, Providers)>);
