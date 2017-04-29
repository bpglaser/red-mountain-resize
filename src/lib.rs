#[macro_use]
extern crate clap;

mod carve;
mod config;

pub use self::carve::run;
pub use self::config::ArgConfig;
pub use self::config::parse_args;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;
