pub trait CanBuildInto<Builder>: Sized {
    type Remainder;

    fn build_into(self, builder: Builder) -> Self::Remainder;
}

// trait FieldsBuilder<Context, Builder> {
//     type Remainder;

//     fn build_field()
