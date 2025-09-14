use cgp::core::field::traits::{static_chars, MaybeChars};
use cgp::prelude::*;

pub const TEST_STR: &'static str = const {
    const TEST: &'static [u8; 3] = &static_chars::<3, Symbol!("abc")>();

    match str::from_utf8(TEST) {
        Ok(value) => value,
        Err(_) => panic!("error"),
    }
};

pub trait TestB<const LEN: usize> {
    const CHARS: &'static [u8];
}

impl<T, const LEN: usize> TestB<LEN> for T
where
    T: MaybeChars,
{
    const CHARS: &'static [u8] = &static_chars::<LEN, T>();
}

pub trait TestC<const LEN: usize> {
    const CHARS: &'static str;
}

impl<T, const LEN: usize> TestC<LEN> for T
where
    T: TestB<LEN>,
{
    const CHARS: &'static str = const {
        match str::from_utf8(T::CHARS) {
            Ok(value) => value,
            Err(_) => panic!("error"),
        }
    };
}

#[test]
fn test_static_chars() {
    assert_eq!(<Symbol!("abc") as TestC<3>>::CHARS, "abc");
}
