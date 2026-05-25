use cgp::core::component::RedirectLookup;
use cgp::prelude::*;

cgp_namespace! {
    new ExtendedNamespace: DefaultNamespace {

        // @cgp.core.error >> @app,
        @cgp.core.error =>
            @app,
    }
}

// cgp_namespace! {
//     ExtendedNamespace: DefaultNamespace {
//         // @cgp.core.error >> @app,
//         @cgp.core.error:
//             @app,
//     }
// }
