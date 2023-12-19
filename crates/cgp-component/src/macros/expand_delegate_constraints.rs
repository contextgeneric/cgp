#[macro_export]
macro_rules! expand_delegate_constraints {
    (
        @target( $target:ty )
        @body(  )
        @head_buf( $( $head:tt )* )
        @tail_buf( $( $tail:tt )* )
    ) => {
        $( $head )*
        $( $tail )*
    };
    (
        @target( $target:ty )
        @body(
            [ ] : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
        @head_buf( $( $head:tt )* )
        @tail_buf( $( $tail:tt )* )
    ) => {
        $crate::expand_delegate_constraints!(
            @target( $target )
            @body(
                $( $( $rest )*  )?
            )
            @head_buf( $( $head )* )
            @tail_buf( $( $tail )* )
        );
    };
    (
        @target( $target:ty )
        @body(
            [ $name:ty $(, $($names:tt)* )?] : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
        @head_buf( $( $head:tt )* )
        @tail_buf( $( $tail:tt )* )
    ) => {
        $crate::expand_delegate_constraints!(
            @target( $target )
            @body(
                $name : $forwarded,
                [ $( $( $names )* )? ] : $forwarded,
                $( $( $rest )*  )?
            )
            @head_buf( $( $head:tt )* )
            @tail_buf( $( $tail )* )
        );
    };
    (
        @target( $target:ty )
        @body(
            $name:ty : $forwarded:ty
            $( , $( $rest:tt )* )?
        )
        @head_buf( $( $head:tt )* )
        @tail_buf( $( $tail:tt )* )
    ) => {
        $crate::expand_delegate_constraints!(
            @target( $target )
            @body(
                $( $( $rest )*  )?
            )
            @head_buf(
                $( $head )*
                + $crate::traits::delegate_component::DelegateComponent< $name, Delegate = $target >
            )
            @tail_buf( $( $tail )* )
        );
    };
}
