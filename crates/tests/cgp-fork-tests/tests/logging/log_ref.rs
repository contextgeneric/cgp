//! `#[cgp_auto_log]` blanket-implements a logger trait whose arguments are
//! references. The detail struct carries that lifetime, and the context bound
//! is `for<'a> CanLog<__LogHello<'a>>`.
//!
//! This is the canonical expansion snapshot for `#[cgp_auto_log]`.
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_log.md.

use std::cell::RefCell;

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_auto_log;

snapshot_cgp_auto_log! {
    #[cgp_auto_log]
    pub trait CanLogHello {
        fn log_hello(&self, name: &str);
    }

    expand_can_log_hello(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait CanLogHello {
            fn log_hello(&self, name: &str);
        }
        pub struct __LogHello<'a> {
            pub name: &'a str,
        }
        impl<__Context__> CanLogHello for __Context__
        where
            __Context__: for<'a> CanLog<__LogHello<'a>>,
        {
            fn log_hello(&self, name: &str) {
                self.log(__LogHello { name })
            }
        }
        "#);
    }
}

pub struct App {
    pub lines: RefCell<Vec<String>>,
}

pub struct LogHelloToLines;

#[cgp_provider(LoggerComponent)]
impl<'a> Logger<App, __LogHello<'a>> for LogHelloToLines {
    fn log(context: &App, detail: __LogHello<'a>) {
        context.lines.borrow_mut().push(detail.name.to_owned());
    }
}

delegate_components! {
    App {
        LoggerComponent: LogHelloToLines,
    }
}

#[test]
fn test_auto_log_ref() {
    let app = App {
        lines: RefCell::new(Vec::new()),
    };

    app.log_hello("Ada");

    assert_eq!(app.lines.borrow().as_slice(), ["Ada"]);
}
