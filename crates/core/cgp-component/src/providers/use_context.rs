use crate::WithProvider;

/// The provider that implements a provider trait by calling the context's own consumer trait,
/// usually as a higher-order provider's default inner provider.
///
/// See [`UseContext`](https://contextgeneric.dev/docs/reference/providers/use_context).
pub struct UseContext;

/// The alias WithProvider`<UseContext>`: a named type or getter component answered by the context's
/// own HasType or HasField entry, keyed by the component.
///
/// See [`WithContext`](https://contextgeneric.dev/docs/reference/providers/with_context).
pub type WithContext = WithProvider<UseContext>;
