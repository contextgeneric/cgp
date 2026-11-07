use cgp::macro_prelude::*;
use cgp_macro::cgp_component;

/// CGP's tag-indexed abstract-type component: a context resolves one type per tag through wiring,
/// and generic code names it through TypeOf.
///
/// See [`HasType`](https://contextgeneric.dev/docs/reference/components/has_type).
#[cgp_component(TypeProvider)]
#[derive_delegate(UseDelegate<Tag>)]
pub trait HasType<Tag> {
    /// The type the context chooses for the tag.
    type Type;
}

/// The type a context chooses for a tag.
pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
