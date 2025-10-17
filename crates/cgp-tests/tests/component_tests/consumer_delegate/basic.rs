use cgp::prelude::*;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_getter]
pub trait HasCount {
    fn count(&self) -> u32;
}

#[derive(HasField)]
pub struct App {
    pub name: String,
    pub count: u32,
}

delegate_components! {
    App {
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}

impl HasCount for App {
    fn count(&self) -> u32 {
        self.count
    }
}

#[test]
fn test_basic_consumer_delegate() {
    let app = App {
        name: "John".to_owned(),
        count: 42,
    };

    assert_eq!(app.name(), "John");
    assert_eq!(app.count(), 42);
}
