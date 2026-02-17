use cgp::prelude::*;

#[cgp_fn]
pub fn capitalize_name(&mut self, #[implicit] name: &mut String) {
    if let Some(first_char) = name.chars().next()
        && first_char.is_lowercase()
    {
        let char_len = first_char.len_utf8();
        let capitalized = first_char.to_uppercase().to_string();
        name.replace_range(..char_len, &capitalized);
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

pub trait CheckPerson: CapitalizeName {}
impl CheckPerson for Person {}
