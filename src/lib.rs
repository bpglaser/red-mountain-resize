#[macro_use]
extern crate clap;
extern crate image;
extern crate itertools;
extern crate num_traits;

pub mod carve;
pub mod config;
pub mod energy;
pub mod grid;
mod point_stack;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;
