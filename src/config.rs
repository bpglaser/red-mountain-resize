use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use clap::Parser;

use crate::change::Change;

#[derive(Debug, Parser)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    #[arg(long)]
    pub width: Option<Change>,
    #[arg(long)]
    pub height: Option<Change>,
    #[arg(long, value_parser = Config::parse_dimensions)]
    pub dimensions: Option<(Change, Change)>,
    #[arg(long)]
    pub debug_path: Option<PathBuf>,
}

impl Config {
    pub fn get_output_path(&mut self, suffix: &str) -> PathBuf {
        let mut output_path = self
            .output_path
            .clone()
            .unwrap_or_else(|| self.get_default_path());
        if !suffix.is_empty() {
            let mut file_name = output_path.file_stem().expect("a file name").to_os_string();
            file_name.push("-");
            file_name.push(suffix);
            file_name.push(output_path.extension().expect("a file extension"));
            output_path.set_file_name(file_name);
        }
        output_path
    }

    fn get_default_path(&self) -> PathBuf {
        let mut output_path = self.input_path.clone();

        let mut stem = self.input_path.file_stem().unwrap().to_owned();
        stem.push("-resized");
        output_path.set_file_name(stem);

        let extension = self.input_path.extension().unwrap();
        output_path.set_extension(extension);

        output_path
    }

    fn parse_dimensions(s: &str) -> Result<(Change, Change)> {
        let mut values = s.split("x");
        let x = values
            .next()
            .and_then(|s| s.parse().ok())
            .context("no x dimension")?;
        let y = values
            .next()
            .and_then(|s| s.parse().ok())
            .context("no y dimension")?;
        Ok((x, y))
    }
}
