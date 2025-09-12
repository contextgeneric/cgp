use cgp::core::field::traits::MatchStr;
use cgp::prelude::*;

#[test]
pub fn test_symbol_display() {
    let val = <Symbol!("hello")>::default();
    assert_eq!(val.to_string(), "hello");

    assert!(<Symbol!("hello")>::match_str("hello"));
    assert!(!<Symbol!("hello")>::match_str("hell"));
}

#[test]
pub fn test_index_display() {
    let val: Index<123> = Default::default();
    assert_eq!(val.to_string(), "123");
}
