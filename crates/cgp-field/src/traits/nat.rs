use core::marker::PhantomData;

pub trait Nat {
    const VALUE: usize;
}

pub struct Z;

pub struct S<N>(pub PhantomData<N>);

impl Nat for Z {
    const VALUE: usize = 0;
}

impl<N: Nat> Nat for S<N> {
    const VALUE: usize = N::VALUE + 1;
}
