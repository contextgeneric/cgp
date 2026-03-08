use cgp::core::field::types;
use cgp::prelude::*;

#[cgp_component(Parser)]
#[use_type(HasErrorType::Error)]
pub trait CanParse<Code, Input> {
    type Output;

    fn run(&self, code: Code, input: &mut Input) -> Result<Self::Output, Error>;
}

pub struct And<A, B>(pub A, pub B);

pub struct Just<T>(pub T);

pub struct Fail<E>(pub E);

#[cgp_impl(new AndDefault)]
#[use_type(HasErrorType::Error)]
impl<A, B, Input, OutputA, OutputB> Parser<And<A, B>, Input>
where
    Self: CanParse<A, Input, Output = OutputA>,
    Self: CanParse<B, Input, Output = OutputB>,
{
    type Output = And<OutputA, OutputB>;

    fn run(&self, code: And<A, B>, input: &mut Input) -> Result<Self::Output, Error> {
        let a = self.run(code.0, input)?;
        let b = self.run(code.1, input)?;
        Ok(And(a, b))
    }
}

#[cgp_impl(new AndJust)]
#[use_type(HasErrorType::Error)]
impl<A, B, Input> Parser<And<Just<A>, Just<B>>, Input> {
    type Output = Just<(A, B)>;

    fn run(
        &self,
        And(Just(a), Just(b)): And<Just<A>, Just<B>>,
        _input: &mut Input,
    ) -> Result<Self::Output, Error> {
        Ok(Just((a, b)))
    }
}

#[cgp_impl(new AndFail)]
impl<T, Input, Error> Parser<And<Just<T>, Fail<Error>>, Input>
where
    // Note: there is a bug in `#[cgp_impl]` so we need to bind this explicitly temporarily
    Self: HasErrorType<Error = Error>,
{
    type Output = Fail<Self::Error>;

    fn run(
        &self,
        And(Just(_), Fail(e)): And<Just<T>, Fail<Error>>,
        _input: &mut Input,
    ) -> Result<Self::Output, Self::Error> {
        Ok(Fail(e))
    }
}

pub struct ParseHttpPath;
pub struct HttpPath;

#[cgp_impl(ParseHttpPath)]
#[use_type(HasErrorType::Error)]
impl Parser<ParseHttpPath, String> {
    type Output = HttpPath;

    fn run(&self, _code: ParseHttpPath, _input: &mut String) -> Result<Self::Output, Error> {
        // Dummmy
        Ok(HttpPath)
    }
}

pub struct ParseHttpQuery;
pub struct HttpQuery;

#[cgp_impl(ParseHttpQuery)]
#[use_type(HasErrorType::Error)]
impl Parser<ParseHttpQuery, String> {
    type Output = HttpQuery;

    fn run(&self, _code: ParseHttpQuery, _input: &mut String) -> Result<Self::Output, Error> {
        // Dummmy
        Ok(HttpQuery)
    }
}

// pub struct ComposeParsers;

// #[cgp_impl(ComposeParsers)]
// #[use_type(HasErrorType::Error)]
// #[use_provider(
//     A: Parser<Input>
// )]
// impl<A, B, Input, OutputA, OutputB> Parser<Cons<A, B>, Input>
// {
//     type Output = (OutputA, OutputB);

//     fn run(
//         &self,
//         _code: PhantomData<Cons<A, B>>,
//         input: &mut Input,
//     ) -> Result<Self::Output, Error> {
//         let a = self.run(PhantomData::<A>, input)?;
//         let b = self.run(PhantomData::<B>, input)?;
//         Ok((a, b))
//     }
// }

// #[cgp_impl(ComposeParsers)]
// #[use_type(HasErrorType::Error)]
// impl<Input> Parser<Nil, Input> {
//     type Output = ();

//     fn run(&self, _code: PhantomData<Nil>, _input: &mut Input) -> Result<Self::Output, Error> {
//         Ok(())
//     }
// }

// #[cgp_impl(new ParseScheme)]
// #[use_type(HasErrorType::Error)]
// impl<Code> Parser<Code, String> {
//     type Output = String;

//     fn run(&self, _code: PhantomData<Code>, _input: &mut String) -> Result<Self::Output, Error> {
//         todo!()
//     }
// }

// pub struct Just<T>(pub T);
// pub

// pub struct And<A, B>(pub A, pub B);

// #[cgp_impl(new AndJust)]
// #[use_type(HasErrorType::Error)]
// impl<A, B, Input> Parser<And<A, B>, Input>
// where
// {

// }
