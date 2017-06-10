extern crate image;
extern crate rmr;

use rmr::carve::Carver;

macro_rules! setup_carver {
    ( $bytes:expr ) => {
        {
            let input = image::load_from_memory($bytes).unwrap();
            let mut carver = Carver::new(&input);
            carver.calculate_energy();
            carver
        }
    };
}

#[test]
fn carver_small_pixel_energy_test() {
    let carver = setup_carver!(SMALL);
    let pixel_energy = carver.get_pixel_energy();
    assert_eq!(get_small_pixel_energy(), pixel_energy);
}

#[test]
fn carver_small_path_energy_test() {
    let carver = setup_carver!(SMALL);
    let path_energy = carver.get_path_energy();
    assert_eq!(get_small_path_energy(), path_energy);
}

#[test]
fn carver_small_get_path_start_test() {
    let carver = setup_carver!(SMALL);
    assert_eq!((0, 3), carver.get_path_start());
}

#[test]
fn carver_small_find_path_test() {
    let carver = setup_carver!(SMALL);
    let (x, y) = carver.get_path_start();
    assert_eq!(get_small_path(), carver.find_path(x, y));
}

#[test]
fn carver_medium_pixel_energy_test() {
    let carver = setup_carver!(MEDIUM);
    let pixel_energy = carver.get_pixel_energy();
    assert_eq!(get_medium_pixel_energy(), pixel_energy);
}

#[test]
fn carver_medium_path_energy_test() {
    let carver = setup_carver!(MEDIUM);
    let path_energy = carver.get_path_energy();
    assert_eq!(get_medium_path_energy(), path_energy);
}

#[test]
fn carver_medium_get_path_start_test() {
    let carver = setup_carver!(MEDIUM);
    assert_eq!((2, 4), carver.get_path_start());
}

#[test]
fn carver_medium_find_path_test() {
    let carver = setup_carver!(MEDIUM);
    let (x, y) = carver.get_path_start();
    assert_eq!(get_medium_path(), carver.find_path(x, y));
}

static SMALL: &'static [u8; 173] = include_bytes!("images/small_energy.png");
static MEDIUM: &'static [u8; 244] = include_bytes!("images/medium_energy.png");

fn get_small_pixel_energy() -> Vec<Vec<u32>> {
    vec![vec![20808, 52020, 20808],
         vec![20808, 52225, 21220],
         vec![20809, 52024, 20809],
         vec![20808, 52225, 21220]]
}

fn get_small_path_energy() -> Vec<Vec<u32>> {
    vec![vec![20808, 52020, 20808],
         vec![41616, 73033, 42028],
         vec![62425, 93640, 62837],
         vec![83233, 114650, 84057]]
}

fn get_small_path() -> Vec<(usize, usize)> {
    vec![(0, 3), (0, 2), (0, 1), (0, 0)]
}

fn get_medium_pixel_energy() -> Vec<Vec<u32>> {
    vec![vec![57685, 50893, 91370, 25418, 33055, 37246],
         vec![15421, 56334, 22808, 54796, 11641, 25496],
         vec![12344, 19236, 52030, 17708, 44735, 20663],
         vec![17074, 23678, 30279, 80663, 37831, 45595],
         vec![32337, 30796, 4909, 73334, 40613, 36556]]
}

fn get_medium_path_energy() -> Vec<Vec<u32>> {
    vec![vec![57685, 50893, 91370, 25418, 33055, 37246],
         vec![66314, 107227, 48226, 80214, 37059, 58551],
         vec![78658, 67462, 100256, 54767, 81794, 57722],
         vec![84536, 91140, 85046, 135430, 92598, 103317],
         vec![116873, 115332, 89955, 158380, 133211, 129154]]
}

fn get_medium_path() -> Vec<(usize, usize)> {
    vec![(2, 4), (2, 3), (3, 2), (4, 1), (3, 0)]
}
