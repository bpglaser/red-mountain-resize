extern crate image;
extern crate rmr;

use rmr::carve::Carver;

#[test]
fn carver_energy_test() {
    let input = image::load_from_memory(INPUT).unwrap();

    let mut carver = Carver::new(&input);
    carver.calculate_energy();

    let energy = carver.get_pixel_energy();
    assert_eq!(precalculated_energy(), energy);
}

static INPUT: &'static [u8; 173] = include_bytes!("images/energy.png");

fn precalculated_energy() -> Vec<Vec<u32>> {
    vec![vec![20808, 52020, 20808],
         vec![20808, 52225, 21220],
         vec![20809, 52024, 20809],
         vec![20808, 52225, 21220]]
}
