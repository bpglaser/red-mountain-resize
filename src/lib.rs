#[macro_use]
extern crate clap;
extern crate image;
extern crate num_traits;

mod carve;
mod config;
mod grid;
pub mod math;
mod point;

pub use carve::run;
pub use config::ArgConfig;
pub use config::parse_args;
pub use grid::Grid;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;
