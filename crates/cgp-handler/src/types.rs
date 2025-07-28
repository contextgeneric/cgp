use core::marker::PhantomData;

#[allow(non_upper_case_globals)]
pub const NoCode: PhantomData<()> = PhantomData;

pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
