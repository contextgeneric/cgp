use core::fmt::Debug;

use cgp_component::{DelegateComponent, HasProvider, IsProviderFor, WithProvider};
use cgp_macro::cgp_type;
use cgp_type::{ProvideType, UseType};

/**
    This is used for contexts to declare that they have a _unique_ `Self::Error` type.

    Although it is possible for each context to declare their own associated
    `Error` type, doing so may result in having multiple ambiguous `Self::Error` types,
    if there are multiple associated types with the same name in different traits.

    As a result, it is better for context traits to include `HasError` as their
    parent traits, so that multiple traits can all refer to the same abstract
    `Self::Error` type.

   The `Error` associated type is also required to implement [`Debug`].
   This is to allow `Self::Error` to be used in calls like `.unwrap()`,
   as well as for simpler error logging.

   More details about how to use `HasErrorType` is available at
   <https://patterns.contextgeneric.dev/error-handling.html>
*/
#[cgp_type]
pub trait HasErrorType {
    type Error: Debug;
}
