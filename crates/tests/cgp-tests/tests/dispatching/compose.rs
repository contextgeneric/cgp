//! Composing `ComputerRef` providers into a combined provider.
//!
//! `FirstNameToString` and `LastNameToString` each read one field of the context;
//! `ConcatOutputs<ProviderA, ProviderB>` is a higher-order `ComputerRef` provider
//! that runs two inner providers and concatenates their outputs. `FullNameToString`
//! is the composition of the two field readers. The plain-function versions
//! (`first_name_to_string`, …, `concate_outputs`) show the same composition
//! written with closures rather than CGP providers.
//!
//! `#[cgp_new_provider]` is incidental scaffolding here (its expansion is owned by
//! `basic_delegation`), so it is written plainly rather than snapshotted.
//!
//! See docs/reference/providers/dispatch_combinators.md and
//! docs/reference/components/computer.md.

use core::fmt::Display;

use cgp::extra::handler::ComputerRef;
use cgp::prelude::*;

pub fn first_name_to_string<Context>(context: &Context) -> String
where
    Context: HasField<Symbol!("first_name"), Value: Display>,
{
    context.get_field(PhantomData).to_string()
}

pub fn last_name_to_string<Context>(context: &Context) -> String
where
    Context: HasField<Symbol!("last_name"), Value: Display>,
{
    context.get_field(PhantomData).to_string()
}

pub fn full_name_to_string<Context>(context: &Context) -> String
where
    Context: HasField<Symbol!("first_name"), Value: Display>
        + HasField<Symbol!("last_name"), Value: Display>,
{
    let composed = concate_outputs(first_name_to_string, last_name_to_string);
    composed(context)
}

pub fn concate_outputs<Context>(
    fn_a: impl Fn(&Context) -> String,
    fn_b: impl Fn(&Context) -> String,
) -> impl Fn(&Context) -> String {
    move |context| format!("{} {}", fn_a(context), fn_b(context))
}

#[cgp_new_provider]
impl<Context, Code, Input> ComputerRef<Context, Code, Input> for FirstNameToString
where
    Context: HasField<Symbol!("first_name"), Value: Display>,
{
    type Output = String;

    fn compute_ref(context: &Context, _code: PhantomData<Code>, _input: &Input) -> String {
        context.get_field(PhantomData).to_string()
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> ComputerRef<Context, Code, Input> for LastNameToString
where
    Context: HasField<Symbol!("last_name"), Value: Display>,
{
    type Output = String;

    fn compute_ref(context: &Context, _code: PhantomData<Code>, _input: &Input) -> String {
        context.get_field(PhantomData).to_string()
    }
}

#[cgp_new_provider]
impl<Context, Code, Input, ProviderA, ProviderB> ComputerRef<Context, Code, Input>
    for ConcatOutputs<ProviderA, ProviderB>
where
    ProviderA: ComputerRef<Context, Code, Input, Output: Display>,
    ProviderB: ComputerRef<Context, Code, Input, Output: Display>,
{
    type Output = String;

    fn compute_ref(context: &Context, code: PhantomData<Code>, input: &Input) -> String {
        let output_a = ProviderA::compute_ref(context, code, input);
        let output_b = ProviderB::compute_ref(context, code, input);
        format!("{output_a} {output_b}")
    }
}

pub type FullNameToString = ConcatOutputs<FirstNameToString, LastNameToString>;
