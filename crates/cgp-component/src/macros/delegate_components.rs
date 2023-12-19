#[macro_export]
macro_rules! delegate_components {
    (   $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            $( @markers[ $( $marker:ident ),* $(,)? ] )?
            ;
    ) => {

    };
    (   $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            $( @markers[ $( $marker:ident ),* $(,)? ] )?
            ;
        [ ] : $forwarded:ty
        $( , $( $rest:tt )* )?
    ) => {
        $crate::delegate_components!(
            $target
                $( < $( $param ),* > )*
                $( @markers[ $( $marker ),* ] )?
                ;
            $( $( $rest )*  )?
        );
    };
    (   $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            $( @markers[ $( $marker:ident ),* $(,)? ] )?
            ;
        [ $name:ty $(, $($names:tt)* )?] : $forwarded:ty
        $( , $( $rest:tt )* )?
    ) => {
        $crate::delegate_component!(
            $target
                $( < $( $param ),* > )*
                $( @markers[ $( $marker ),* ] )?
                ;
            $name : $forwarded
        );

        $crate::delegate_components!(
            $target
                $( < $( $param ),* > )*
                $( @markers[ $( $marker ),* ] )?
                ;
            [ $( $( $names )* )? ] : $forwarded
            $( , $( $rest )*  )?
        );
    };
    (   $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            $( @markers[ $( $marker:ident ),* $(,)? ] )?
            ;
        $name:ty : $forwarded:ty
        $( , $( $rest:tt )* )?
    ) => {
        $crate::delegate_component!(
            $target
                $( < $( $param ),* > )*
                $( @markers[ $( $marker ),* ] )?
                ;
            $name : $forwarded
        );

        $crate::delegate_components!(
            $target
                $( < $( $param ),* > )*
                $( @markers[ $( $marker ),* ] )?
                ;
            $( $( $rest )*  )?
        );
    };
}
