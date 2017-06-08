extern crate rmr;

use rmr::grid::Grid;

// Test grid visualized:
//  -----------
// | 0 | 1 | 2 |
// | 3 | 4 | 5 |
// | 6 | 7 | 8 |
//  -----------

#[test]
fn grid_size_test() {
    let grid = make_test_grid();
    assert_eq!(3, grid.width());
    assert_eq!(3, grid.height());
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

    // Third row
    assert_eq!(&6, grid.get(0, 2));
    assert_eq!(&7, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
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

    // Third row
    assert_eq!(&mut 6, grid.get_mut(0, 2));
    assert_eq!(&mut 7, grid.get_mut(1, 2));
    assert_eq!(&mut 8, grid.get_mut(2, 2));
}

#[test]
fn grid_coord_iter_test() {
    let grid = make_test_grid();
    let mut iter = grid.coord_iter();

    // First row
    assert_eq!((0, 0, &0), iter.next().unwrap());
    assert_eq!((1, 0, &1), iter.next().unwrap());
    assert_eq!((2, 0, &2), iter.next().unwrap());

    // Second row
    assert_eq!((0, 1, &3), iter.next().unwrap());
    assert_eq!((1, 1, &4), iter.next().unwrap());
    assert_eq!((2, 1, &5), iter.next().unwrap());

    // Third row
    assert_eq!((0, 2, &6), iter.next().unwrap());
    assert_eq!((1, 2, &7), iter.next().unwrap());
    assert_eq!((2, 2, &8), iter.next().unwrap());

    assert!(iter.next().is_none());
}

#[test]
fn grid_get_adjacent_test() {
    let grid = make_test_grid();

    // First row
    let (left, right, up, down) = grid.get_adjacent(0, 0);
    assert_eq!(&2, left);
    assert_eq!(&1, right);
    assert_eq!(&6, up);
    assert_eq!(&3, down);

    let (left, right, up, down) = grid.get_adjacent(1, 0);
    assert_eq!(&0, left);
    assert_eq!(&2, right);
    assert_eq!(&7, up);
    assert_eq!(&4, down);

    let (left, right, up, down) = grid.get_adjacent(2, 0);
    assert_eq!(&1, left);
    assert_eq!(&0, right);
    assert_eq!(&8, up);
    assert_eq!(&5, down);

    // Second row
    let (left, right, up, down) = grid.get_adjacent(0, 1);
    assert_eq!(&5, left);
    assert_eq!(&4, right);
    assert_eq!(&0, up);
    assert_eq!(&6, down);

    let (left, right, up, down) = grid.get_adjacent(1, 1);
    assert_eq!(&3, left);
    assert_eq!(&5, right);
    assert_eq!(&1, up);
    assert_eq!(&7, down);

    let (left, right, up, down) = grid.get_adjacent(2, 1);
    assert_eq!(&4, left);
    assert_eq!(&3, right);
    assert_eq!(&2, up);
    assert_eq!(&8, down);

    // Third row
    let (left, right, up, down) = grid.get_adjacent(0, 2);
    assert_eq!(&8, left);
    assert_eq!(&7, right);
    assert_eq!(&3, up);
    assert_eq!(&0, down);

    let (left, right, up, down) = grid.get_adjacent(1, 2);
    assert_eq!(&6, left);
    assert_eq!(&8, right);
    assert_eq!(&4, up);
    assert_eq!(&1, down);

    let (left, right, up, down) = grid.get_adjacent(2, 2);
    assert_eq!(&7, left);
    assert_eq!(&6, right);
    assert_eq!(&5, up);
    assert_eq!(&2, down);
}

#[test]
fn grid_get_parents_test() {
    let grid = make_test_grid();

    // First row
    assert!(grid.get_parents(0, 0).iter().all(Option::is_none));
    assert!(grid.get_parents(1, 0).iter().all(Option::is_none));
    assert!(grid.get_parents(2, 0).iter().all(Option::is_none));

    // Second row
    assert_eq!([None, Some(&0), Some(&1)], grid.get_parents(0, 1));
    assert_eq!([Some(&0), Some(&1), Some(&2)], grid.get_parents(1, 1));
    assert_eq!([Some(&1), Some(&2), None], grid.get_parents(2, 1));

    // Third row
    assert_eq!([None, Some(&3), Some(&4)], grid.get_parents(0, 2));
    assert_eq!([Some(&3), Some(&4), Some(&5)], grid.get_parents(1, 2));
    assert_eq!([Some(&4), Some(&5), None], grid.get_parents(2, 2));
}

#[test]
fn grid_get_parents_indexed_test() {
    let grid = make_test_grid();

    // First row
    assert!(grid.get_parents_indexed(0, 0).is_empty());
    assert!(grid.get_parents_indexed(1, 0).is_empty());
    assert!(grid.get_parents_indexed(2, 0).is_empty());

    // Second row
    assert_eq!(vec![(0, 0, &0), (1, 0, &1)], grid.get_parents_indexed(0, 1));
    assert_eq!(vec![(0, 0, &0), (1, 0, &1), (2, 0, &2)],
               grid.get_parents_indexed(1, 1));
    assert_eq!(vec![(1, 0, &1), (2, 0, &2)], grid.get_parents_indexed(2, 1));

    // Third row
    assert_eq!(vec![(0, 1, &3), (1, 1, &4)], grid.get_parents_indexed(0, 2));
    assert_eq!(vec![(0, 1, &3), (1, 1, &4), (2, 1, &5)],
               grid.get_parents_indexed(1, 2));
    assert_eq!(vec![(1, 1, &4), (2, 1, &5)], grid.get_parents_indexed(2, 2));
}

#[test]
fn grid_get_row_test() {
    let grid = make_test_grid();

    // First row
    assert_eq!(vec![&0, &1, &2], grid.get_row(0));

    // Second row
    assert_eq!(vec![&3, &4, &5], grid.get_row(1));

    // Third row
    assert_eq!(vec![&6, &7, &8], grid.get_row(2));
}

#[test]
fn grid_get_row_with_coords_test() {
    let grid = make_test_grid();

    // First row
    assert_eq!(vec![(0, 0, &0), (1, 0, &1), (2, 0, &2)],
               grid.get_row_with_coords(0));

    // Second row
    assert_eq!(vec![(0, 1, &3), (1, 1, &4), (2, 1, &5)],
               grid.get_row_with_coords(1));

    // Third row
    assert_eq!(vec![(0, 2, &6), (1, 2, &7), (2, 2, &8)],
               grid.get_row_with_coords(2));
}

#[test]
fn grid_shift_row_left_from_point_test() {
    let mut grid = make_test_grid();

    // First row
    grid.shift_row_left_from_point(0, 0);
    assert_eq!(&1, grid.get(0, 0));
    assert_eq!(&2, grid.get(1, 0));
    assert_eq!(&2, grid.get(2, 0));

    // Second row
    grid.shift_row_left_from_point(1, 1);
    assert_eq!(&3, grid.get(0, 1));
    assert_eq!(&5, grid.get(1, 1));
    assert_eq!(&5, grid.get(2, 1));

    // Third row
    grid.shift_row_left_from_point(2, 2);
    assert_eq!(&6, grid.get(0, 2));
    assert_eq!(&7, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
}

#[test]
fn grid_shift_row_right_from_point_test() {
    let mut grid = make_test_grid();

    // First row
    grid.shift_row_right_from_point(0, 0);
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&0, grid.get(1, 0));
    assert_eq!(&1, grid.get(2, 0));

    // Second row
    grid.shift_row_right_from_point(1, 1);
    assert_eq!(&3, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));
    assert_eq!(&4, grid.get(2, 1));

    // Third row
    grid.shift_row_right_from_point(2, 2);
    assert_eq!(&6, grid.get(0, 2));
    assert_eq!(&7, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
}

#[test]
fn grid_remove_last_column_test() {
    let mut grid = make_test_grid();

    grid.remove_last_column();
    assert_eq!(2, grid.width());
    assert_eq!(3, grid.height());
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&7, grid.get(1, 2));

    grid.remove_last_column();
    assert_eq!(1, grid.width());
    assert_eq!(3, grid.height());
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&6, grid.get(0, 2));

    grid.remove_last_column();
    assert_eq!(0, grid.width());
    assert_eq!(0, grid.width());
}

#[test]
fn grid_add_last_column_test() {
    let mut grid = make_test_grid();

    grid.add_last_column();
    assert_eq!(4, grid.width());
    assert_eq!(3, grid.height());

    // First row
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&1, grid.get(1, 0));
    assert_eq!(&2, grid.get(2, 0));
    assert_eq!(&2, grid.get(3, 0));

    // Second row
    assert_eq!(&3, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));
    assert_eq!(&5, grid.get(2, 1));
    assert_eq!(&5, grid.get(3, 1));

    // Third row
    assert_eq!(&6, grid.get(0, 2));
    assert_eq!(&7, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
    assert_eq!(&8, grid.get(3, 2));
}

// Rotated test grid visualized:
//  -----------
// | 0 | 3 | 6 |
// | 1 | 4 | 7 |
// | 2 | 5 | 8 |
//  -----------

#[test]
fn grid_rotation_size_test() {
    let mut grid = make_test_grid();

    grid.rotate();
    assert_eq!(3, grid.width());
    assert_eq!(3, grid.height());

    grid.rotate();
    assert_eq!(3, grid.width());
    assert_eq!(3, grid.height());
}

#[test]
fn grid_rotation_get_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    // First row
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&3, grid.get(1, 0));
    assert_eq!(&6, grid.get(2, 0));

    // Second row
    assert_eq!(&1, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));
    assert_eq!(&7, grid.get(2, 1));

    // Third row
    assert_eq!(&2, grid.get(0, 2));
    assert_eq!(&5, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
}

#[test]
fn grid_rotation_get_mut_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    // First row
    assert_eq!(&mut 0, grid.get_mut(0, 0));
    assert_eq!(&mut 3, grid.get_mut(1, 0));
    assert_eq!(&mut 6, grid.get_mut(2, 0));

    // Second row
    assert_eq!(&mut 1, grid.get_mut(0, 1));
    assert_eq!(&mut 4, grid.get_mut(1, 1));
    assert_eq!(&mut 7, grid.get_mut(2, 1));

    // Third row
    assert_eq!(&mut 2, grid.get_mut(0, 2));
    assert_eq!(&mut 5, grid.get_mut(1, 2));
    assert_eq!(&mut 8, grid.get_mut(2, 2));
}

#[test]
fn grid_rotation_coord_iter_test() {
    let mut grid = make_test_grid();
    grid.rotate();
    let mut iter = grid.coord_iter();

    // First row
    assert_eq!((0, 0, &0), iter.next().unwrap());
    assert_eq!((1, 0, &3), iter.next().unwrap());
    assert_eq!((2, 0, &6), iter.next().unwrap());

    // Second row
    assert_eq!((0, 1, &1), iter.next().unwrap());
    assert_eq!((1, 1, &4), iter.next().unwrap());
    assert_eq!((2, 1, &7), iter.next().unwrap());

    // Third row
    assert_eq!((0, 2, &2), iter.next().unwrap());
    assert_eq!((1, 2, &5), iter.next().unwrap());
    assert_eq!((2, 2, &8), iter.next().unwrap());

    assert!(iter.next().is_none());
}

#[test]
fn grid_rotation_shift_row_left_from_point_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    // First row
    grid.shift_row_left_from_point(0, 0);
    assert_eq!(&3, grid.get(0, 0));
    assert_eq!(&6, grid.get(1, 0));
    assert_eq!(&6, grid.get(2, 0));

    // Second row
    grid.shift_row_left_from_point(1, 1);
    assert_eq!(&1, grid.get(0, 1));
    assert_eq!(&7, grid.get(1, 1));
    assert_eq!(&7, grid.get(2, 1));

    // Third row
    grid.shift_row_left_from_point(2, 2);
    assert_eq!(&2, grid.get(0, 2));
    assert_eq!(&5, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
}

#[test]
fn grid_rotation_shift_row_right_from_point_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    // First row
    grid.shift_row_right_from_point(0, 0);
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&0, grid.get(1, 0));
    assert_eq!(&3, grid.get(2, 0));

    // Second row
    grid.shift_row_right_from_point(1, 1);
    assert_eq!(&1, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));
    assert_eq!(&4, grid.get(2, 1));

    // Third row
    grid.shift_row_right_from_point(2, 2);
    assert_eq!(&2, grid.get(0, 2));
    assert_eq!(&5, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
}

#[test]
fn grid_rotation_remove_last_column_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    grid.remove_last_column();
    assert_eq!(2, grid.width());
    assert_eq!(3, grid.height());
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&5, grid.get(1, 2));

    grid.remove_last_column();
    assert_eq!(1, grid.width());
    assert_eq!(3, grid.height());
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&2, grid.get(0, 2));

    grid.remove_last_column();
    assert_eq!(0, grid.width());
    assert_eq!(0, grid.width());
}

#[test]
fn grid_rotation_add_last_column_test() {
    let mut grid = make_test_grid();
    grid.rotate();

    grid.add_last_column();
    assert_eq!(4, grid.width());
    assert_eq!(3, grid.height());

    // First row
    assert_eq!(&0, grid.get(0, 0));
    assert_eq!(&3, grid.get(1, 0));
    assert_eq!(&6, grid.get(2, 0));
    assert_eq!(&6, grid.get(3, 0));

    // Second row
    assert_eq!(&1, grid.get(0, 1));
    assert_eq!(&4, grid.get(1, 1));
    assert_eq!(&7, grid.get(2, 1));
    assert_eq!(&7, grid.get(3, 1));

    // Third row
    assert_eq!(&2, grid.get(0, 2));
    assert_eq!(&5, grid.get(1, 2));
    assert_eq!(&8, grid.get(2, 2));
    assert_eq!(&8, grid.get(3, 2));
}

fn make_test_grid() -> Grid<isize> {
    Grid::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]])
}
