use core::marker::PhantomData;

pub struct PathCons<Head: ?Sized, Tail: ?Sized>(pub PhantomData<Head>, pub PhantomData<Tail>);

pub struct PathNil;

pub trait ConcatPath<Other: ?Sized> {
    type Output: ?Sized;
}

impl<Head: ?Sized, Tail: ?Sized, Other: ?Sized> ConcatPath<Other> for PathCons<Head, Tail>
where
    Tail: ConcatPath<Other>,
{
    type Output = PathCons<Head, <Tail as ConcatPath<Other>>::Output>;
}

impl<Other: ?Sized> ConcatPath<Other> for PathNil {
    type Output = Other;
}
