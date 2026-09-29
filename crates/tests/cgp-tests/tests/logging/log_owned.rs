//! `#[cgp_auto_log]` on a method whose arguments are owned. The detail struct
//! has no lifetime, and the context bound is `CanLog<__LogCount>` rather than a
//! higher-ranked bound.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_log.md.

use std::cell::Cell;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_log;

snapshot_cgp_auto_log! {
    #[cgp_auto_log]
    pub trait CanLogCount {
        fn log_count(&self, count: u64);
    }

    expand_can_log_count(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait CanLogCount {
            fn log_count(&self, count: u64);
        }
        pub struct __LogCount {
            pub count: u64,
        }
        impl<__Context__> CanLogCount for __Context__
        where
            __Context__: CanLog<__LogCount>,
        {
            fn log_count(&self, count: u64) {
                self.log(__LogCount { count })
            }
        }
        "#);
    }
}

pub struct App {
    pub count: Cell<u64>,
}

pub struct LogCountToCell;

#[cgp_provider(LoggerComponent)]
impl Logger<App, __LogCount> for LogCountToCell {
    fn log(context: &App, detail: __LogCount) {
        context.count.set(detail.count);
    }
}

delegate_components! {
    App {
        LoggerComponent: LogCountToCell,
    }
}

#[test]
fn test_auto_log_owned() {
    let app = App {
        count: Cell::new(0),
    };

    app.log_count(7);

    assert_eq!(app.count.get(), 7);
}
