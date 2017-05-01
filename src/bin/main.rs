extern crate seam_carving_resize;

use std::process::exit;

use seam_carving_resize::*;

fn main() {
    match parse_args().and_then(run) {
        Ok(_) => exit(0),
        Err(error) => {
            println!("{:?}", error.description());
            exit(1)
        }
    }
}
