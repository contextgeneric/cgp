use cgp::prelude::*;

#[test]
pub fn test_symbol_display() {
    let val = <symbol!("hello")>::default();
    assert_eq!(val.to_string(), "hello");
}

#[test]
pub fn test_index_display() {
    let val: Index<123> = Default::default();
    assert_eq!(val.to_string(), "123");
}
