/// Name the concrete struct a partial record will become, at every configuration, so generic code
/// can mention the destination mid-build.
///
/// See [`PartialData`](https://contextgeneric.dev/docs/reference/traits/builder/partial_data).
pub trait PartialData {
    /// The target the partial value builds.
    type Target;
}
