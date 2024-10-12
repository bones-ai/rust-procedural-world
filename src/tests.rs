#[cfg(test)]
use crate::grid::*;

#[test]
fn test_maze() {
    let maze = Grid::new_maze(8, 12, 1234);

    assert_eq!(maze.width(), 8);
    assert_eq!(maze.height(), 12);
    assert_eq!(maze.max_x_index(), 7);
    assert_eq!(maze.max_y_index(), 11);
}

#[test]
fn test_is_wall_between() {
    let maze = Grid::new_maze(2, 2, 1234);

    // Vertical wall between two cells
    assert_eq!(maze.is_wall_between((1, 0), (0, 0)), true);
    assert_eq!(maze.is_wall_between((0, 0), (1, 0)), true);

    // Horizontal wall between two cells
    assert_eq!(maze.is_wall_between((0, 1), (0, 0)), false);
    assert_eq!(maze.is_wall_between((0, 0), (0, 1)), false);
}
