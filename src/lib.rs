#[macro_use]
extern crate clap;
extern crate image;
extern crate num_traits;

pub mod carve;
pub mod config;
pub mod energy;
pub mod grid;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;
