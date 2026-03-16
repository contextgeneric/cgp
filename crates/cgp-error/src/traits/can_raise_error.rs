use core::marker::PhantomData;

use cgp_component::*;
use cgp_macro::{cgp_component, delegate_components};

use crate::traits::has_error_type::HasErrorType;

/**
   The `CanRaiseError` trait is used to raise any concrete error type into
   an abstract error provided by [`HasErrorType`].
*/
#[cgp_component {
    provider: ErrorRaiser,
    derive_delegate: UseDelegate<SourceError>,
}]
pub trait CanRaiseError<SourceError>: HasErrorType {
    fn raise_error(error: SourceError) -> Self::Error;
}

pub struct ErrorRaiserComponents<Provider>(pub PhantomData<Provider>);

delegate_components! {
    UseDefault {
        ErrorRaiserComponent:
            UseDelegate<ErrorRaiserComponents<UseDefault>>,
    }
}

impl DelegateComponent1<ErrorRaiserComponent, &'static str> for UseDefault {
    type Delegate = ();
}

// delegate_components! {
//     ErrorRaiserComponents<UseDefault> {
//         &'static str:
//             RaiseFrom,
//     }
// }
