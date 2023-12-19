#[macro_export]
macro_rules! delegate_all {
    (
        $source_marker:ident,
        $source:ty,
        $target:ty
            $(,)?
    ) => {
        impl<Component> DelegateComponent<Component> for $target
        where
            Self: $source_marker<Component>,
        {
            type Delegate = $source;
        }
    };
}
