/// A partially built value and its target.
///
/// See [`PartialData`](https://contextgeneric.dev/docs/reference/traits/builder/partial_data).
pub trait PartialData {
    /// The target the partial value builds.
    type Target;
}
