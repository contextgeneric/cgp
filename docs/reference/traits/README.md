# Runtime traits (pending)

This directory will hold reference documents for the CGP capability and mechanism traits that the macros expand into — the traits a programmer rarely writes by hand but must understand to read generated code. None are written yet; their semantics currently live inside the macro documents that generate them.

The planned documents, in rough priority order, are the mechanism traits that nearly every macro references — `delegate_component.md` (`DelegateComponent`), `is_provider_for.md` (`IsProviderFor`), and `can_use_component.md` (`CanUseComponent`) — followed by the capability traits `has_field.md` (`HasField`), `has_fields.md` (`HasFields`), `has_type.md` (`HasType` / `TypeProvider`), and `has_error_type.md` (`HasErrorType` / `CanRaiseError`). See the [reference index](../README.md) for how these fit the whole catalog, and [../../CLAUDE.md](../../CLAUDE.md) for the authoring rules every document here must follow.
