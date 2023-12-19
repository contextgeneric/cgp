#[macro_export]
macro_rules! delegate_components {
    (
        $( @marker( $marker:ident ) )?
        $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            ;
        $( $rest:tt )*
    ) => {
        $crate::delegate_components!(
            $( @marker( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body( $( $rest )* )
        );
    };
    (
        $( @marker( $marker:ident ) )?
        @target(
            $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        )
        @body(  )
    ) => {

    };
    (
        $( @marker( $marker:ident ) )?
        @target(
            $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        )
        @body(
            [ ] : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
    ) => {
        $crate::delegate_components!(
            $( @marker( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body(
                $( $( $rest )*  )?
            )
        );
    };
    (
        $( @marker( $marker:ident ) )?
        @target(
            $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        )
        @body(
            [ $name:ty $(, $($names:tt)* )?] : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
    ) => {
        $crate::delegate_component!(
            $target
                $( < $( $param ),* > )*
                ;
            $name : $forwarded
        );

        $( impl<T> $marker < $name > for T {} )?

        $crate::delegate_components!(
            $( @marker( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body(
                [ $( $( $names )* )? ] : $forwarded
                $( , $( $rest )*  )?
            )
        );
    };
    (
        $( @marker( $marker:ident ) )?
        @target(
            $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        )
        @body(
            $name:ty : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
    ) => {
        $crate::delegate_component!(
            $target
                $( < $( $param ),* > )*
                ;
            $name : $forwarded
        );

        $( impl<T> $marker < $name > for T {} )?

        $crate::delegate_components!(
            $( @marker( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body(
                $( $( $rest )*  )?
            )
        );
    };
}
