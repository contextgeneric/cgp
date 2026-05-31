#[derive(Clone, Eq, PartialEq)]
pub enum FieldMode {
    Reference,
    OptionRef,
    MRef,
    Str,
    Copy,
    Slice,
}
