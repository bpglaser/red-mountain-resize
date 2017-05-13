extern crate seam_carving_resize;

use seam_carving_resize::grid::Grid;

// Test grid visualized:
//  -----------
// | 0 | 1 | 2 |
// | 3 | 4 | 5 |
//  -----------

#[test]
fn grid_size_test() {
    let grid = make_test_grid();
    assert_eq!(3, grid.width());
    assert_eq!(2, grid.height());
}

#[test]
fn grid_get_test() {
    let grid = make_test_grid();

    // First row
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&1, grid.get(1, 0));
    assert_eq!(&2, grid.get(2, 0));

    // Second row
    assert_eq!(&3, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));
    assert_eq!(&5, grid.get(2, 1));
}

#[test]
fn grid_get_mut_test() {
    let mut grid = make_test_grid();

    // First row
    assert_eq!(&mut 0, grid.get_mut(0, 0));
    assert_eq!(&mut 1, grid.get_mut(1, 0));
    assert_eq!(&mut 2, grid.get_mut(2, 0));

    // Second row
    assert_eq!(&mut 3, grid.get_mut(0, 1));
    assert_eq!(&mut 4, grid.get_mut(1, 1));
    assert_eq!(&mut 5, grid.get_mut(2, 1));
}

// Rotated test grid visualized:
//  -------
// | 0 | 3 |
// | 1 | 4 |
// | 2 | 5 |
//  -------

#[test]
fn grid_rotation_size_test() {
    let mut grid = make_test_grid();
    grid.rotate();
    assert_eq!(2, grid.width());
    assert_eq!(3, grid.height());
    grid.rotate();
    assert_eq!(3, grid.width());
    assert_eq!(2, grid.height());
}

#[test]
fn grid_rotation_get_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    // First row
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&3, grid.get(1, 0));

    // Second row
    assert_eq!(&1, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));

    // Third row
    assert_eq!(&2, grid.get(0, 2));
    assert_eq!(&5, grid.get(1, 2));
}

#[test]
fn grid_rotation_get_mut_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    // First row
    assert_eq!(&mut 0, grid.get_mut(0, 0));
    assert_eq!(&mut 3, grid.get_mut(1, 0));

    // Second row
    assert_eq!(&mut 1, grid.get_mut(0, 1));
    assert_eq!(&mut 4, grid.get_mut(1, 1));

    // Third row
    assert_eq!(&mut 2, grid.get_mut(0, 2));
    assert_eq!(&mut 5, grid.get_mut(1, 2));
}

fn make_test_grid() -> Grid<isize> {
    Grid::new(vec![vec![0, 1, 2], vec![3, 4, 5]])
}
