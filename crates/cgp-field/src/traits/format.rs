use core::fmt::{self, Formatter};

use crate::types::{Nil, Symbol};

pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}

pub trait MaybeChars {
    const LEN: usize;

    const VALUE: Option<char>;

    type Next: MaybeChars;
}

pub trait StaticBytes {
    const BYTES: &'static [u8];
}

impl StaticBytes for Nil {
    const BYTES: &'static [u8] = &[];
}

impl<const LEN: usize, Chars> StaticBytes for Symbol<LEN, Chars>
where
    Chars: MaybeChars,
{
    const BYTES: &'static [u8] = &static_chars::<LEN, Chars>();
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
