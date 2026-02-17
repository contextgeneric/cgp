// pub fn derive_implicit_arg_constraint(
//     field: &ImplicitArgField,
// ) -> syn::Result<TypeParamBound> {
//     let field_symbol = &field.field_name;
//     let field_type = &field.field_type;

//     let constraint = if field.field_mut.is_none() {
//         if let FieldMode::Slice = field.field_mode {
//             quote! {
//                 HasField< #field_symbol, Value: AsRef< [ #field_type ] > + 'static >
//             }
//         } else {
//             quote! {
//                 HasField< #field_symbol, Value = #field_type >
//             }
//         }
//     } else {
//         quote! {
//             HasFieldMut< #field_symbol, Value = #field_type >
//         }
//     };

//     parse2(constraint)
// }
