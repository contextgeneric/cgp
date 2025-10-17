use cgp::prelude::*;

#[cgp_getter {
    provider: ValueGetter,
    name: ValueGetterComponent<Value>,
}]
pub trait HasValue<Value> {
    fn value(&self, _tag: PhantomData<Value>) -> &Value;
}

#[derive(HasField)]
pub struct App {
    pub name: String,
    pub count: u32,
}

delegate_components! {
    App {
        ValueGetterComponent<String>: UseField<Symbol!("name")>,
    }
}

impl HasValue<u32> for App {
    fn value(&self, _tag: PhantomData<u32>) -> &u32 {
        &self.count
    }
}

#[test]
fn test_generic_consumer_delegate() {
    let app = App {
        name: "John".to_owned(),
        count: 42,
    };

    assert_eq!(app.value(PhantomData::<String>), "John");
    assert_eq!(app.value(PhantomData::<u32>), &42);
}
