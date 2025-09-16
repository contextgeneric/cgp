use cgp::core::field::traits::StaticString;
use cgp::prelude::*;

#[test]
fn test_static_chars() {
    assert_eq!(<Symbol!("") as StaticString>::VALUE, "");
    assert_eq!(<Symbol!("a") as StaticString>::VALUE, "a");
    assert_eq!(<Symbol!("abc") as StaticString>::VALUE, "abc");
    assert_eq!(<Symbol!("世界你好") as StaticString>::VALUE, "世界你好");
    assert_eq!(
        <Symbol!("a quick brown fox jumps over the lazy dog") as StaticString>::VALUE,
        "a quick brown fox jumps over the lazy dog",
    );
}
