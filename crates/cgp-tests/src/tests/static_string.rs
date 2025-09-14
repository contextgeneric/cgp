use core::mem::transmute;

use cgp::core::field::traits::{static_chars, MaybeChars, Nat, StaticString};
use cgp::prelude::*;

pub const TEST_STR: &'static str = const {
    const TEST: &'static [u8; 3] = &static_chars::<3, Symbol!("abc")>();

    match str::from_utf8(TEST) {
        Ok(value) => value,
        Err(_) => panic!("error"),
    }
};

// pub const TEST_A: [u8; 3] = const {
//     unsafe {
//         transmute((1u8, (2u8, 3u8)))
//     }
// };

// pub const TEST_A: &'static (u8, (u8, (u8, ()))) = &(1, (2, (3, ())));

// pub const TEST_B: &'static [u8] = const {
//     unsafe {
//         transmute(TEST_A)
//     }
// };

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

// pub trait TestD {
//     const CHARS: &'static str;
// }

// impl<T, N: Nat> TestD for T
// where
//     T: TestC<{N::VALUE}> + MaybeChars<Len = N>,
// {
//     const CHARS: &'static str = T::CHARS;
// }

#[test]
fn test_static_chars() {
    // assert_eq!(TEST_A, [1, 2, 3]);

    assert_eq!(<Symbol!("abc") as StaticString>::VALUE, "abc");
}
