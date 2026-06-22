use crate::types::check_components::CheckEntries;

pub trait ToCheckEntries {
    fn to_check_entries(&self) -> syn::Result<CheckEntries>;
}
