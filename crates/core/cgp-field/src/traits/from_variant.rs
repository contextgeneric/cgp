use core::marker::PhantomData;

/// Construct an enum from a single variant chosen by its type-level name, so generic code can build
/// a variant it cannot name as a constructor.
///
/// See [`FromVariant`](https://contextgeneric.dev/docs/reference/traits/variant/from_variant).
pub trait FromVariant<Tag> {
    /// The variant's payload type.
    type Value;

    /// Builds the variant.
    fn from_variant(_tag: PhantomData<Tag>, value: Self::Value) -> Self;
}
