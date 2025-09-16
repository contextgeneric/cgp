use core::marker::PhantomData;

pub struct Symbol<const LEN: usize, Bytes>(pub PhantomData<Bytes>);
