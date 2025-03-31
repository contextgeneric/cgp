use core::marker::PhantomData;

#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct ι<const CHAR: char, Tail>(pub PhantomData<Tail>);

pub use ι as Char;
