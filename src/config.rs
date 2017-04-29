use std::path::PathBuf;

use clap::ArgMatches;

use BoxResult;

pub fn parse_args() -> BoxResult<ArgConfig> {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Brad Glaser <bpglaser@gmail.com")
        (about: "foobar")
        (@arg file_path: <filename>)
    )
            .get_matches();
    ArgConfig::try_from(matches)
}

#[derive(Debug)]
pub struct ArgConfig {
    pub file_path: PathBuf,
    pub direction: Direction,
}

impl ArgConfig {
    fn try_from(matches: ArgMatches) -> BoxResult<Self> {
        let file_path = matches
            .value_of("file_path")
            .ok_or("No file path given.")?
            .into();
        let direction = Direction::Horizontal(150); // todo implement
        Ok(ArgConfig {
               file_path,
               direction,
           })
    }
}

#[derive(Debug)]
pub enum Direction {
    Horizontal(usize),
    Vertical(usize),
}
