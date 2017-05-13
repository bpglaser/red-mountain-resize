#[macro_use]
extern crate clap;
extern crate image;
extern crate num_traits;

pub mod carve;
pub mod config;
pub mod grid;
pub mod math;
pub mod point;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;
