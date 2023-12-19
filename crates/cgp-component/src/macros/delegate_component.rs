#[macro_export]
macro_rules! delegate_component {
    (
        $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        {
            $name:ty : $forwarded:ty $(,)?
        }
    ) => {
        impl< $( $( $param ),* )* >
            $crate::traits::delegate_component::DelegateComponent< $name >
            for $target $( < $( $param ),* > )*
        where
            Self: $crate::traits::sync::Async,
        {
            type Delegate = $forwarded;
        }
    };
}
