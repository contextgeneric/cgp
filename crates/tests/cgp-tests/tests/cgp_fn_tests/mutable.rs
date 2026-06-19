use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
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

    expand_capitalize_name(output) {
        insta::assert_snapshot!(output, @"
        pub trait CapitalizeName {
            fn capitalize_name(&mut self);
        }
        impl<__Context__> CapitalizeName for __Context__
        where
            Self: HasFieldMut<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = String,
            >,
        {
            fn capitalize_name(&mut self) {
                let name: &mut String = self
                    .get_field_mut(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    );
                if let Some(first_char) = name.chars().next() && first_char.is_lowercase() {
                    let char_len = first_char.len_utf8();
                    let capitalized = first_char.to_uppercase().to_string();
                    name.replace_range(..char_len, &capitalized);
                }
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

pub trait CheckPerson: CapitalizeName {}
impl CheckPerson for Person {}
