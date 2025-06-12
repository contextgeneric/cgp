/**
    This trait is used by the blanket implementations of CGP consumer traits to
    forward the implementation to the `CgpProvider` type, which impements the
    corresponding provider trait.

    The `HasCgpProvider` trait is automatically implemented by `#[cgp_context]`
    when defining a context type, together with the generated provider struct.

    Typically, the `CgpProvider` would contain provider implementation mapping
    through `delegate_component!` macro to implement the corresponding provider
    traits. However, it is also possible to implement context-specific provider
    traits on the `CgpProvider` type directly.

    ## Example

    Given the following context definition:

    ```rust,ignore
    #[cgp_context(MyContextComponents)]
    pub struct MyContext {
        ...
    }
    ```

    The following would be generated:

    ```rust,ignore
    pub struct MyContextComponents;

    impl HasCgpProvider for MyContext {
        type CgpProvider = MyContextComponents;
    }
    ```
*/
pub trait HasCgpProvider {
    type CgpProvider;
}
