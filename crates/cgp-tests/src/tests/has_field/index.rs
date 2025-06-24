use cgp::prelude::*;

#[derive(HasField)]
pub struct Context(pub String, pub u64);

pub trait CheckHasFieldImpls:
    HasField<Index<0>, Value = String> + HasField<Index<1>, Value = u64>
{
}

impl CheckHasFieldImpls for Context {}

#[test]
fn test_has_field_index() {
    let context = Context("test".to_owned(), 1);
    assert_eq!(context.get_field(PhantomData::<Index<0>>), &"test");
    assert_eq!(context.get_field(PhantomData::<Index<1>>), &1);
}
