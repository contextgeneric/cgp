#[derive(Clone, Eq, PartialEq)]
pub enum FieldMode {
    Reference,
    OptionRef,
    OptionStr,
    MRef,
    Str,
    Copy,
    Slice,
}
