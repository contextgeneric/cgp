use syn::token::Mut;
use syn::{Ident, Type};

use crate::types::getter::FieldMode;

/// One getter method parsed into the field it reads.
#[derive(Clone)]
pub struct GetterField {
    /// The field's name.
    pub field_name: Ident,
    /// The field's type.
    pub field_type: Type,
    /// The getter's return type.
    pub return_type: Type,
    /// The `mut` of a `&mut self` receiver.
    pub receiver_mut: Option<Mut>,
    /// The type of a phantom argument, when the getter takes one.
    pub phantom_arg_type: Option<Type>,
    /// How the field is read.
    pub field_mode: FieldMode,
    /// How the getter takes its receiver.
    pub receiver_mode: ReceiverMode,
}

/// How a getter takes its receiver.
#[derive(Clone)]
pub enum ReceiverMode {
    /// The getter takes `self`.
    SelfReceiver,
    /// The getter takes a typed receiver.
    Type(Box<Type>),
}
