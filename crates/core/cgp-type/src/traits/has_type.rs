use cgp::macro_prelude::*;
use cgp_macro::cgp_component;

#[cgp_component(TypeProvider)]
#[derive_delegate(UseDelegate<Tag>)]
/// A type the context chooses behind a tag.
///
/// See [`HasType`](https://contextgeneric.dev/docs/reference/components/has_type).
pub trait HasType<Tag> {
    /// The type the context chooses for the tag.
    type Type;
}

/// The type a context chooses for a tag.
pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
