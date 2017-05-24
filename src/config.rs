use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};

use BoxResult;

pub fn parse_args() -> BoxResult<Config> {
    let matches = App::new("Seam Carving Resize")
        .version("0.1.0")
        .author("Brad Glaser <bpglaser@gmail.com>")
        .arg(Arg::with_name("distance")
                 .long("distance")
                 .required(true)
                 .takes_value(true)
                 .validator(validate_distance))
        .arg(Arg::with_name("horizontal")
                 .long("horizontal")
                 .required_unless("vertical")
                 .conflicts_with("vertical"))
        .arg(Arg::with_name("vertical")
                 .long("vertical")
                 .required_unless("horizontal")
                 .conflicts_with("horizontal"))
        .arg(Arg::with_name("grow")
                 .long("grow")
                 .required_unless("shrink")
                 .conflicts_with("shrink"))
        .arg(Arg::with_name("shrink")
                 .long("shrink")
                 .required_unless("grow")
                 .conflicts_with("grow"))
        .arg(Arg::with_name("debug").long("debug"))
        .arg(Arg::with_name("file_path").required(true))
        .arg(Arg::with_name("save_path").required(true))
        .get_matches();

    Config::try_from(matches)
}

fn validate_distance(s: String) -> Result<(), String> {
    s.parse::<usize>()
        .map(|_| ())
        .map_err(|_| "Invalid distance provided.".to_owned())
}

#[derive(Debug)]
pub struct Config {
    pub file_path: PathBuf,
    pub save_path: PathBuf,
    pub distance: usize,
    pub orientation: Orientation,
    pub mode: Mode,
    pub save_path_image: bool,
}

impl Config {
    fn try_from(matches: ArgMatches) -> BoxResult<Self> {
        let file_path = matches
            .value_of("file_path")
            .ok_or("No file path given.")?
            .into();

        let save_path = matches
            .value_of("save_path")
            .ok_or("No save path given.")?
            .into();

        let distance = matches
            .value_of("distance")
            .ok_or("No distance given.")?
            .parse()?;

        let orientation = if matches.is_present("horizontal") {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        };

        let mode = if matches.is_present("grow") {
            Mode::Grow
        } else {
            Mode::Shrink
        };

        let save_path_image = matches.is_present("debug");

        Ok(Self {
               file_path,
               save_path,
               distance,
               orientation,
               mode,
               save_path_image,
           })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Grow,
    Shrink,
}
