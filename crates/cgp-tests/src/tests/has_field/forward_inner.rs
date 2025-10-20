use cgp::prelude::*;

#[derive(HasField)]
pub struct Foo {
    pub foo_value: String,
    pub bar: Bar,
}

#[derive(HasField)]
pub struct Bar {
    pub bar_value: u64,
    pub baz: Baz,
}

#[derive(HasField)]
pub struct Baz {
    pub baz_value: i32,
}

impl<Tag, Value> HasField<Tag> for Foo
where
    Bar: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value {
        self.bar.get_field(tag)
    }
}

impl<Tag, Value> HasField<Tag> for Bar
where
    Baz: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(&self, tag: PhantomData<Tag>) -> &Self::Value {
        self.baz.get_field(tag)
    }
}

#[cgp_auto_getter]
pub trait HasFooValue {
    fn foo_value(&self) -> &str;
}

#[cgp_auto_getter]
pub trait HasBarValue {
    fn bar_value(&self) -> u64;
}

#[cgp_auto_getter]
pub trait HasBazValue {
    fn baz_value(&self) -> i32;
}

#[test]
fn test_bulk_forward_has_field() {
    let context = Foo {
        foo_value: "hello".to_owned(),
        bar: Bar {
            bar_value: 42,
            baz: Baz { baz_value: 128 },
        },
    };

    assert_eq!(
        context.get_field(PhantomData::<Symbol!("foo_value")>),
        "hello"
    );
    assert_eq!(context.foo_value(), "hello");

    assert_eq!(context.get_field(PhantomData::<Symbol!("bar_value")>), &42);
    assert_eq!(context.bar_value(), 42);

    assert_eq!(context.get_field(PhantomData::<Symbol!("baz_value")>), &128);
    assert_eq!(context.baz_value(), 128);
}
