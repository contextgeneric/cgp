#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Either<Head, Tail> {
    Left(Head),
    Right(Tail),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Void {}
