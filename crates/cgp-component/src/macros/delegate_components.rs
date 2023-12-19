#[macro_export]
macro_rules! delegate_components {
    (
        @mark_component( $marker:ident )
        @mark_delegate( $delegate_marker:ident )
        $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            ;
        $( $rest:tt )*
    ) => {
        $crate::delegate_components!(
            @mark_component( $marker )
            @target( $target $( < $( $param ),* > )? )
            @body( $( $rest )* )
        );

        pub trait $marker < Component > {}

        $crate::expand_delegate_constraints!(
            @target( $target $( < $( $param ),* > )? )
            @body( $( $rest )* )
            @head_buf(
                pub trait $delegate_marker: Sized
            )
            @tail_buf(
                {}
            )
        );

        $crate::expand_delegate_constraints!(
            @target( $target $( < $( $param ),* > )? )
            @body( $( $rest )* )
            @head_buf(
                impl<Components> $delegate_marker for Components
                where
                    Components: Sized
            )
            @tail_buf(
                {}
            )
        );

    };
    (
        $( @mark_component( $marker:ident ) )?
        $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            ;
        $( $rest:tt )*
    ) => {
        $(
            pub trait $marker < Component > {}
        )?

        $crate::delegate_components!(
            $( @mark_component( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body( $( $rest )* )
        );
    };
    (
        $( @mark_component( $marker:ident ) )?
        @target(
            $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        )
        @body(  )
    ) => {

    };
    (
        $( @mark_component( $marker:ident ) )?
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
            $( @mark_component( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body(
                $( $( $rest )*  )?
            )
        );
    };
    (
        $( @mark_component( $marker:ident ) )?
        @target(
            $target:ident
            $( < $( $param:ident ),* $(,)? > )?
        )
        @body(
            [ $name:ty $(, $($names:tt)* )?] : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
    ) => {
        $crate::delegate_components!(
            $( @mark_component( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body(
                $name : $forwarded,
                [ $( $( $names )* )? ] : $forwarded
                $( , $( $rest )*  )?
            )
        );
    };
    (
        $( @mark_component( $marker:ident ) )?
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
            $( @mark_component( $marker ) )?
            @target( $target $( < $( $param ),* > )? )
            @body(
                $( $( $rest )*  )?
            )
        );
    };
}
