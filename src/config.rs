use std::path::PathBuf;

use clap::ArgMatches;

use BoxResult;

pub fn parse_args() -> BoxResult<Config> {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Brad Glaser <bpglaser@gmail.com")
        (about: "foobar")
        (@arg file_path: <input_filename>)
        (@arg save_path: <output_filename>)
    )
            .get_matches();
    Config::try_from(matches)
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Config {
    pub file_path: PathBuf,
    pub save_path: PathBuf,
    pub distance: isize,
    pub orientation: Orientation,
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

        let distance = -100; // todo implement
        let orientation = Orientation::Horizontal; // todo implement
        Ok(Self {
               file_path,
               save_path,
               distance,
               orientation,
           })
    }
}
