use cgp::prelude::*;

/// Logs one `Detail` value.
///
/// `Detail` is chosen by the caller: a message, a structured event, or a detail
/// struct produced by `#[cgp_auto_log]`. A context wires `LoggerComponent` to the
/// provider that accepts the details it cares about.
#[cgp_component(Logger)]
#[prefix(@cgp.extra.log in DefaultNamespace)]
pub trait CanLog<Detail> {
    fn log(&self, detail: Detail);
}
