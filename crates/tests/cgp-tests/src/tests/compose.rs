use core::fmt::Display;

use cgp::extra::handler::{ComputerRef, ComputerRefComponent};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_new_provider;

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

snapshot_cgp_new_provider! {
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

    expand_first_name_to_string(output) {
        insta::assert_snapshot!(output, @r#"
        impl<Context, Code, Input> ComputerRef<Context, Code, Input> for FirstNameToString
        where
            Context: HasField<Symbol!("first_name"), Value: Display>,
        {
            type Output = String;
            fn compute_ref(
                context: &Context,
                _code: PhantomData<Code>,
                _input: &Input,
            ) -> String {
                context.get_field(PhantomData).to_string()
            }
        }
        impl<Context, Code, Input> IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
        for FirstNameToString
        where
            Context: HasField<Symbol!("first_name"), Value: Display>,
        {}
        pub struct FirstNameToString;
        "#)
    }
}

snapshot_cgp_new_provider! {
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

    expand_last_name_to_string(output) {
        insta::assert_snapshot!(output, @r#"
        impl<Context, Code, Input> ComputerRef<Context, Code, Input> for LastNameToString
        where
            Context: HasField<Symbol!("last_name"), Value: Display>,
        {
            type Output = String;
            fn compute_ref(
                context: &Context,
                _code: PhantomData<Code>,
                _input: &Input,
            ) -> String {
                context.get_field(PhantomData).to_string()
            }
        }
        impl<Context, Code, Input> IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
        for LastNameToString
        where
            Context: HasField<Symbol!("last_name"), Value: Display>,
        {}
        pub struct LastNameToString;
        "#)
    }
}

snapshot_cgp_new_provider! {
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

    expand_concat_outputs(output) {
        insta::assert_snapshot!(output, @r#"
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
        impl<
            Context,
            Code,
            Input,
            ProviderA,
            ProviderB,
        > IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
        for ConcatOutputs<ProviderA, ProviderB>
        where
            ProviderA: IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
                + ComputerRef<Context, Code, Input, Output: Display>,
            ProviderB: IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
                + ComputerRef<Context, Code, Input, Output: Display>,
        {}
        pub struct ConcatOutputs<ProviderA, ProviderB>(
            pub ::core::marker::PhantomData<(ProviderA, ProviderB)>,
        );
        "#)
    }
}

pub type FullNameToString = ConcatOutputs<FirstNameToString, LastNameToString>;
