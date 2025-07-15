#![no_std]

pub mod prelude;

pub use {
    cgp_dispatch as dispatch, cgp_error_extra as error, cgp_handler as handler, cgp_inner as inner,
    cgp_monad as monad, cgp_run as run, cgp_runtime as runtime,
};
