#[diagnostic::on_unimplemented(
    message = "{Self} does not contain any DelegateComponent entry for {Name}",
    note = "You might want to implement the provider trait for {Name} on {Self}"
)]
pub trait DelegateComponent<Name> {
    type Delegate;
}
