use syn::token::Mut;
use syn::{Ident, Type};

pub struct GetterField {
    pub field_name: Ident,
    pub field_type: Type,
    pub return_type: Type,
    pub field_mut: Option<Mut>,
    pub phantom_arg_type: Option<Type>,
    pub field_mode: FieldMode,
    pub receiver_mode: ReceiverMode,
}

pub enum FieldMode {
    Reference,
    OptionRef,
    MRef,
    Str,
    Clone,
    Slice,
}

pub enum ReceiverMode {
    SelfReceiver,
    Type(Box<Type>),
}
