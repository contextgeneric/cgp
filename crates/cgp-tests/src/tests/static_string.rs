use cgp::core::field::traits::StaticString;
use cgp::prelude::*;

#[test]
fn test_static_chars() {
    assert_eq!(<Symbol!("abc") as StaticString>::VALUE, "abc");
}
