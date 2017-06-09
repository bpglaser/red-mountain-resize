extern crate image;
extern crate rmr;

use rmr::carve::Carver;

#[test]
fn carver_small_energy_test() {
    let input = image::load_from_memory(SMALL).unwrap();

    let mut carver = Carver::new(&input);
    carver.calculate_energy();

    let energy = carver.get_pixel_energy();
    assert_eq!(get_small_energy(), energy);
}

#[test]
fn carver_medium_energy_test() {
    let input = image::load_from_memory(MEDIUM).unwrap();

    let mut carver = Carver::new(&input);
    carver.calculate_energy();

    let energy = carver.get_pixel_energy();
    assert_eq!(get_medium_energy(), energy);
}

static SMALL: &'static [u8; 173] = include_bytes!("images/small_energy.png");
static MEDIUM: &'static [u8; 244] = include_bytes!("images/medium_energy.png");

fn get_small_energy() -> Vec<Vec<u32>> {
    vec![vec![20808, 52020, 20808],
         vec![20808, 52225, 21220],
         vec![20809, 52024, 20809],
         vec![20808, 52225, 21220]]
}

fn get_medium_energy() -> Vec<Vec<u32>> {
    vec![vec![57685, 50893, 91370, 25418, 33055, 37246],
         vec![15421, 56334, 22808, 54796, 11641, 25496],
         vec![12344, 19236, 52030, 17708, 44735, 20663],
         vec![17074, 23678, 30279, 80663, 37831, 45595],
         vec![32337, 30796, 4909, 73334, 40613, 36556]]
}
