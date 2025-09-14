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

pub trait Grow {
    type Grown;
}

// impl<const I: usize> Grow for [u8; I]
// {
//     type Grown = [u8; I + 1];
// }

pub trait ByteArray {
    type Bytes;
}

// impl<N: Nat> ByteArray for N {
//     type Bytes = [u8; N::VALUE];
// }

// impl ByteArray for Z {
//     type Bytes = [u8; 0];
// }

// impl<const LEN: usize, N> ByteArray for S<N>
// where
//     N: ByteArray<Bytes = [u8; LEN]>,
// {
//     type Bytes = [u8; LEN];
// }
