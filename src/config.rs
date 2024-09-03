use std::path::{Path, PathBuf};

#[derive(Debug, clap::Parser)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    #[arg(long)]
    pub width: Option<isize>,
    #[arg(long)]
    pub height: Option<isize>,
    #[arg(long, value_parser = Config::parse_dimensions)]
    pub dimensions: Option<(usize, usize)>,
    #[arg(long)]
    pub debug_path: Option<PathBuf>,
    #[arg(long)]
    pub time: bool,
}

impl Config {
    pub fn get_output_path(&mut self) -> &Path {
        match self.output_path {
            Some(ref output_path) => output_path,
            None => self.get_default_path(),
        }
    }

    fn get_default_path(&mut self) -> &Path {
        let mut output_path = self.input_path.clone();

        let mut stem = self.input_path.file_stem().unwrap().to_owned();
        stem.push("-resized");
        output_path.set_file_name(stem);

        let extension = self.input_path.extension().unwrap();
        output_path.set_extension(extension);

        self.output_path = Some(output_path);
        self.output_path.as_ref().unwrap()
    }

    fn parse_dimensions(s: &str) -> Result<(usize, usize), &'static str> {
        let mut values = s.split("x");
        let x = values
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or("no x dimension")?;
        let y = values
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or("no y dimension")?;
        Ok((x, y))
    }
}
