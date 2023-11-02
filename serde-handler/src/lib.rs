pub mod channel;
#[cfg(feature = "missing_closure_type")]
mod missing_closure_type;
#[cfg(feature = "multiple_handlers1")]
mod multiple_handlers1;
#[cfg(feature = "multiple_handlers2")]
mod multiple_handlers2;
#[cfg(feature = "multiple_handlers3")]
mod multiple_handlers3;
#[cfg(feature = "multiple_handlers4")]
mod multiple_handlers4;
#[cfg(feature = "start")]
mod start;
#[cfg(feature = "working")]
pub mod working;
#[cfg(feature = "zero_copy1")]
mod zero_copy1;
#[cfg(feature = "zero_copy2")]
pub mod zero_copy2;
#[cfg(feature = "zero_copy3")]
pub mod zero_copy3;
#[cfg(feature = "zero_copy4")]
pub mod zero_copy4;

type Result<T, E = String> = std::result::Result<T, E>;
