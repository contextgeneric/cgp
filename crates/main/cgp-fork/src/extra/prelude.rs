pub use cgp_fork_macro::{cgp_auto_dispatch, cgp_auto_log, cgp_computer, cgp_producer};

pub use crate::extra::dispatch::{
    MatchFirstWithValueHandlers, MatchFirstWithValueHandlersMut, MatchFirstWithValueHandlersRef,
    MatchWithValueHandlers, MatchWithValueHandlersMut, MatchWithValueHandlersRef,
};
pub use crate::extra::handler::{
    AsyncComputer, AsyncComputerComponent, AsyncComputerRef, AsyncComputerRefComponent, Computer,
    ComputerComponent, ComputerRefComponent, Handler, HandlerComponent, HandlerRefComponent,
    Producer, ProducerComponent, PromoteAsyncComputer, PromoteComputer, PromoteHandler,
    PromoteProducer, PromoteTryComputer, TryComputer, TryComputerComponent,
    TryComputerRefComponent,
};
pub use crate::extra::log::{CanLog, Logger, LoggerComponent};
