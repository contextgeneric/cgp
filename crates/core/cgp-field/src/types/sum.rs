/// The head-or-rest cell of the type-level sum list, which describes an enum one variant at a time
/// so generic code can match any enum's variants.
///
/// See [`Either`](https://contextgeneric.dev/docs/reference/types/either).
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Either<Head, Tail> {
    /// The first alternative.
    Left(Head),
    /// The remaining alternatives.
    Right(Tail),
}

/// The uninhabited end marker of the sum list: an empty enum that can never be constructed, which
/// lets a complete variant match end without a runtime branch.
///
/// See [`Void`](https://contextgeneric.dev/docs/reference/types/void).
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Void {}
