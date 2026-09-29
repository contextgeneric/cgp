//! `#[cgp_auto_error]` turns one inherent impl into the three error providers:
//! `ErrorTypeProvider`, `ErrorRaiser`, and `ErrorWrapper`. The method bodies are
//! the providers; a bare `Error` in type position is the abstract error.
//!
//! This is the canonical expansion snapshot for `#[cgp_auto_error]`.
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_error.md.

use core::fmt::Display;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_error;

#[derive(Debug, PartialEq, Eq)]
pub struct AppError {
    pub message: String,
}

snapshot_cgp_auto_error! {
    #[cgp_auto_error]
    impl ProvideAppError {
        type Error = AppError;

        fn raise_error<Source>(source: Source) -> Error
        where
            Source: Display,
        {
            AppError {
                message: source.to_string(),
            }
        }

        fn wrap_error<Detail>(error: Error, detail: Detail) -> Error
        where
            Detail: Display,
        {
            AppError {
                message: format!("{detail}: {}", error.message),
            }
        }
    }

    expand_provide_app_error(output) {
        insta::assert_snapshot!(output, @r#"
        pub struct ProvideAppError;
        impl<__Context__> ErrorTypeProvider<__Context__> for ProvideAppError {
            type Error = AppError;
        }
        impl<__Context__> IsProviderFor<ErrorTypeProviderComponent, __Context__, ()>
        for ProvideAppError {}
        impl<__Context__, Source> ErrorRaiser<__Context__, Source> for ProvideAppError
        where
            __Context__: HasErrorType<Error = AppError>,
            Source: Display,
        {
            #[track_caller]
            fn raise_error(source: Source) -> __Context__::Error {
                AppError {
                    message: source.to_string(),
                }
            }
        }
        impl<__Context__, Source> IsProviderFor<ErrorRaiserComponent, __Context__, (Source)>
        for ProvideAppError
        where
            __Context__: HasErrorType<Error = AppError>,
            Source: Display,
        {}
        impl<__Context__, Detail> ErrorWrapper<__Context__, Detail> for ProvideAppError
        where
            __Context__: HasErrorType<Error = AppError>,
            Detail: Display,
        {
            #[track_caller]
            fn wrap_error(error: __Context__::Error, detail: Detail) -> __Context__::Error {
                AppError {
                    message: format!("{detail}: {}", error.message),
                }
            }
        }
        impl<__Context__, Detail> IsProviderFor<ErrorWrapperComponent, __Context__, (Detail)>
        for ProvideAppError
        where
            __Context__: HasErrorType<Error = AppError>,
            Detail: Display,
        {}
        "#);
    }
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: ProvideAppError,
        ErrorRaiserComponent: ProvideAppError,
        ErrorWrapperComponent: ProvideAppError,
    }
}

check_components! {
    App {
        ErrorTypeProviderComponent,
        ErrorRaiserComponent: &'static str,
        ErrorWrapperComponent: &'static str,
    }
}

#[test]
fn test_auto_error_raise_and_wrap() {
    let error: AppError = App::raise_error("missing");
    assert_eq!(error.message, "missing");

    let error = App::wrap_error(error, "while loading");
    assert_eq!(error.message, "while loading: missing");
}
