extern crate seam_carving_resize;

use seam_carving_resize::*;

fn main() {
    parse_args().and_then(run).unwrap()
}

fn run(config: Config) -> BoxResult<()> {
    unimplemented!()
}
