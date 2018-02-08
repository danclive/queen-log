#[macro_use]
extern crate lazy_static;
extern crate chrono;
//#[macro_use]
mod macros;
mod logger;
mod queue;
pub mod color;

pub use logger::{LOG, MAX_LEVEL};
pub use logger::{Logger, DefaultLogger};
pub use logger::{Log, init, Level, Record, Metadata};
