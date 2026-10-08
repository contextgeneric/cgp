use core::marker::PhantomData;

pub struct Life<'a>(pub PhantomData<*mut &'a ()>);
