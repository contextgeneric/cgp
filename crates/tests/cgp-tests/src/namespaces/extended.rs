use cgp::prelude::{DefaultNamespace, cgp_namespace};

cgp_namespace! {
    new ExtendedNamespace: DefaultNamespace {
        @cgp.core.error =>
            @app,
    }
}
