use crate::types::{Char, Nil, Symbol};

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
            Err(_) => panic!("error const decoding &[u8] to &str"),
        }
    };
}

trait StaticBytes {
    const BYTES: &'static [u8];
}

trait MaybeChars {
    const LEN: usize;

    const VALUE: Option<char>;

    type Next: MaybeChars;
}

impl<const LEN: usize, Chars> StaticBytes for Symbol<LEN, Chars>
where
    Chars: MaybeChars,
{
    const BYTES: &'static [u8] = &static_chars::<LEN, Chars>();
}

impl StaticBytes for Nil {
    const BYTES: &'static [u8] = &[];
}

impl<const CHAR: char, Tail> MaybeChars for Char<CHAR, Tail>
where
    Tail: MaybeChars,
{
    const LEN: usize = Tail::LEN + 1;

    const VALUE: Option<char> = Some(CHAR);

    type Next = Tail;
}

impl MaybeChars for Nil {
    const LEN: usize = 0;

    const VALUE: Option<char> = None;

    type Next = Nil;
}

const fn static_chars<const LEN: usize, S: MaybeChars>() -> [u8; LEN] {
    let mut chars = [0; LEN];

    update_chars::<S>(&mut chars);

    chars
}

const fn update_chars<S: MaybeChars>(mut chars: &mut [u8]) {
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
