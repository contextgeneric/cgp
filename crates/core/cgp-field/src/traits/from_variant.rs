use core::marker::PhantomData;

/// Builds an enum variant from its payload.
///
/// See [`FromVariant`](https://contextgeneric.dev/docs/reference/traits/variant/from_variant).
pub trait FromVariant<Tag> {
    /// The variant's payload type.
    type Value;

    /// Builds the variant.
    fn from_variant(_tag: PhantomData<Tag>, value: Self::Value) -> Self;
}
