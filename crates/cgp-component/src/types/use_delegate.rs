use core::marker::PhantomData;

/**
    The `UseDelegate` pattern is used as the _default dispatcher_ for CGP
    components that contain additional generic parameters in their traits.

    When a provider trait contains additional generic parameters in addition
    to the `Context` type, CGP can generate a `UseDelegate` implementation
    that uses `Components` as a _type-level lookup table_ to dispatch the
    implementation to different providers based on the generic types.

    The implementation of `UseDelegate` follows the same pattern as the
    blanket implementation of a provider trait. However, instead of using
    the component name type as the key, it uses the specified generic
    parameters as the key to lookup the provider through `DelegateComponent`.

    `UseDelegate` is very commonly used to perform ad hoc dispatch of
    concrete types to different context-generic providers. This allows the
    providers to remain generic, even when they may have implementations
    that overlaps on the generic parameters.

    The implementation of `UseDelegate` can be automatically generated through
    the `derive_delegate` entry in `#[cgp_component]`. It is also possible to
    implement the dispatcher pattern on types other than `UseDelegate`, especially
    when there are multiple generic parameters that could be dispatched differently.
    We mainly use `UseDelegate` as the default dispatcher, so that users don't need
    to remember the different provider types to be used with each component.

    ## Example

    Given the following component definition:

    ```rust,ignore
    #[cgp_component {
        provider: ErrorRaiser,
        derive_delegate: UseDelegate<SourceError>,
    }]
    pub trait CanRaiseError<SourceError>: HasErrorType {
        fn raise_error(error: SourceError) -> Self::Error;
    }
    ```

    The following `UseDelegate` implementation would be generated:

    ```rust,ignore
    impl<Context, SourceError, Components, Delegate> ErrorRaiser<Context, SourceError>
        for UseDelegate<Components>
    where
        Context: HasErrorType,
        Components: DelegateComponent<(SourceError), Delegate = Delegate>,
        Delegate: ErrorRaiser<Context, SourceError>,
    {
        fn raise_error(error: SourceError) -> Context::Error {
            Delegate::raise_error(error)
        }
    }
    ```
*/
pub struct UseDelegate<Components>(pub PhantomData<Components>);
