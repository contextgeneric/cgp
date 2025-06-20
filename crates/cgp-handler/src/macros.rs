#[macro_export]
macro_rules! cgp_producer {
    (
        $provider:ident: $output:ty {
            $($body:tt)*
        }
    ) => {
        #[cgp_new_provider]
        impl<Context, Code> Producer<Context, Code> for $provider {
            type Output = $output;

            fn produce(_context: &Context, _code: PhantomData<Code>) -> Self::Output {
                $($body)*
            }
        }
    };
}
