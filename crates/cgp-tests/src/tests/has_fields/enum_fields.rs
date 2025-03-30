use cgp::prelude::*;

#[test]
fn test_simple_enum() {
    #[derive(HasFields)]
    pub enum Person {
        Anonymous(u32),
        Named(String),
    }
}
