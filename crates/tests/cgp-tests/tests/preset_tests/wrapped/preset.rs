#[cgp::re_export_imports]
mod preset {
    use core::convert::Infallible;
    use core::error::Error;
    use std::io::Error as IoError;

    use cgp::core::component::UseDelegate;
    use cgp::extra::error::{RaiseFrom, RaiseInfallible, ReturnError};
    use cgp::prelude::*;

    pub type BoxError = Box<dyn Error>;

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        ErrorHandlerPreset {
            BoxError: ReturnError,
            Infallible: RaiseInfallible,
            IoError: RaiseFrom,
        }
    }
}
