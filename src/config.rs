use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};

use BoxResult;

pub fn parse_args() -> BoxResult<Config> {
    let matches = App::new("Red Mountain Resize")
        .version("0.1.0")
        .author("Brad Glaser <bpglaser@gmail.com>")
        .arg(Arg::with_name("width")
                 .short("w")
                 .long("width")
                 .value_name("WIDTH")
                 .takes_value(true)
                 .allow_hyphen_values(true))
        .arg(Arg::with_name("height")
                 .short("h")
                 .long("height")
                 .value_name("HEIGHT")
                 .takes_value(true)
                 .allow_hyphen_values(true))
        .arg(Arg::with_name("dimensions")
                 .short("d")
                 .long("dimensions")
                 .conflicts_with_all(&["width", "height"])
                 .required_unless_one(&["width", "height"])
                 .value_name("WIDTHxHEIGHT")
                 .takes_value(true)
                 .number_of_values(2)
                 .value_delimiter("x"))
        .arg(Arg::with_name("debug")
                 .long("debug")
                 .value_name("DEBUG_PATH")
                 .takes_value(true))
        .arg(Arg::with_name("input_path")
                 .required(true)
                 .value_name("INPUT_PATH")
                 .takes_value(true))
        .arg(Arg::with_name("output_path")
                 .required(false)
                 .value_name("OUTPUT_PATH")
                 .takes_value(true))
        .get_matches();

    // TODO remove me and add input validators
    println!("{:?}", matches);
    panic!();

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
    pub debug_image_path: Option<String>,
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

        let debug_image_path = matches.value_of("debug").map(|s| s.to_owned());

        Ok(Self {
               file_path,
               save_path,
               distance,
               orientation,
               mode,
               debug_image_path,
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
