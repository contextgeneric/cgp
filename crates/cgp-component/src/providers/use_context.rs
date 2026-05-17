use crate::WithProvider;

/**
    The `UseContext` pattern is used to define a trivial implementation of
    a provider trait, by forwarding the implementation to the consumer trait
    implementation of the context.

    This pattern is the _dual_ of the blanket implementation of a consumer trait,
    which forwards the implementation to its provider through the `CgpProvider` trait.

    The main use case for `UseContext` is to be used as a _higher-order provider_
    in the argument to a different provider implementation. This decouples the
    dependencies between different CGP traits, and allows non-default providers
    to be used within another provider implementation.

    For obvious reasons, `UseContext` should not be used as the delegation target
    from the context provider, as it would result in a cyclic dependency error.

    The `UseContext` provider is automatically implemented for all CPG traits that
    are generated from `#[cgp_component]`.

    ## Example

    Given the following component definition:

    ```rust,ignore
    #[cgp_component(Greeter)]
    pub trait CanGreet {
        fn greet(&self);
    }
    ```

    The following `UseContext` implementation would be generated:

    ```rust,ignore
    impl<Context> Greeter<Context> for UseContext
    where
        Context: CanGreet,
    {
        fn greet(context: &Context) {
            context.greet()
        }
    }
    ```
*/
pub struct UseContext;

pub type WithContext = WithProvider<UseContext>;
