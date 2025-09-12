use crate::types::{Char, Nil};

pub trait MatchStr {
    fn match_str(value: &str) -> bool;
}

impl<T> MatchStr for T
where
    T: MatchChars,
{
    fn match_str(value: &str) -> bool {
        T::match_chars(value.chars())
    }
}

trait MatchChars {
    fn match_chars(chars: impl Iterator<Item = char>) -> bool;
}

impl<const CHAR: char, Tail> MatchChars for Char<CHAR, Tail>
where
    Tail: MatchChars,
{
    fn match_chars(mut chars: impl Iterator<Item = char>) -> bool {
        match chars.next() {
            Some(c) => {
                if c != CHAR {
                    false
                } else {
                    Tail::match_chars(chars)
                }
            }
            None => false,
        }
    }
}

impl MatchChars for Nil {
    fn match_chars(mut chars: impl Iterator<Item = char>) -> bool {
        chars.next().is_none()
    }
}
