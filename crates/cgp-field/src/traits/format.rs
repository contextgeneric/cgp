use core::fmt::{self, Display, Formatter};

pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}

pub trait StaticDisplay {
    const VALUE: &'static dyn Display;
}

pub trait MaybeChars {
    const LEN: usize;

    const VALUE: Option<char>;

    type Next: MaybeChars;
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
