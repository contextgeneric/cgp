#[derive(Eq, PartialEq, Debug, Clone)]
pub enum σ<Head, Tail> {
    Left(Head),
    Right(Tail),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum θ {}

pub use {θ as Void, σ as Either};
