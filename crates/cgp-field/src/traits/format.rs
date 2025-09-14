use core::fmt::{self, Display, Formatter};

use crate::traits::Nat;
use crate::types::{Char, Nil};

pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}

pub trait StaticDisplay {
    const VALUE: &'static dyn Display;
}

pub trait MaybeChars {
    type Len: Nat;

    const VALUE: Option<char>;

    type Next: MaybeChars;
}

pub trait StaticBytes {
    const BYTES: &'static [u8];
}

impl StaticBytes for Nil {
    const BYTES: &'static [u8] = &[];
}

// impl<const C: char, R> StaticBytes for Char<C, R>
// where
//     R: MaybeChars,
// {
//     const BYTES: &'static [u8] = &static_chars::<{R::Len::VALUE}, Self>();
// }

impl<const C1: char, const C2: char, const C3: char> StaticBytes
    for Char<C1, Char<C2, Char<C3, Nil>>>
{
    const BYTES: &'static [u8] = &static_chars::<3, Self>();
}

pub trait StaticString {
    const VALUE: &'static str;
}

impl<T> StaticString for T
where
    T: StaticBytes,
{
    const VALUE: &'static str = const {
        match str::from_utf8(T::BYTES) {
            Ok(value) => value,
            Err(_) => panic!("error"),
        }
    };
}

pub const fn static_chars<const LEN: usize, S: MaybeChars>() -> [u8; LEN] {
    let mut chars = [0; LEN];

    update_chars::<S>(&mut chars);

    chars
}

pub const fn update_chars<S: MaybeChars>(mut chars: &mut [u8]) {
    if let Some(value) = S::VALUE {
        value.encode_utf8(chars);

        let len = value.len_utf8();

        let mut j = 0;
        while j < len {
            chars = chars.split_first_mut().unwrap().1;
            j += 1;
        }

        update_chars::<S::Next>(chars);
    }
}
