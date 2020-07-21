#[macro_use]
extern crate clap;

pub mod carve;
pub mod config;
pub mod energy;
pub mod grid;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
