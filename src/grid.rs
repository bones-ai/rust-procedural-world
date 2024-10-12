use bevy::{prelude::*, utils::HashSet};

#[derive(Debug, Clone, Component)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub seed: u32,
}

#[derive(Debug, Clone, Component)]
pub struct Cell {
    pub top_wall: bool,
    pub bottom_wall: bool,
    pub left_wall: bool,
    pub right_wall: bool,
}

impl Grid {
    fn new_empty(width: usize, height: usize, seed: u32) -> Self {
        if height == 0 || width == 0 {
            panic!(
                "maze height and width must be greater than zero. received height: {} and width: {}",
                height, width,
            );
        }
        let cells: Vec<Vec<Cell>> = vec![vec![Cell::new(); width]; height];
        Grid { cells, seed }
    }

    fn new_walled(width: usize, height: usize, seed: u32) -> Self {
        let mut grid = Self::new_empty(width, height, seed);
        for row in &mut grid.cells {
            for cell in row {
                cell.top_wall = true;
                cell.bottom_wall = true;
                cell.left_wall = true;
                cell.right_wall = true;
            }
        }
        grid
    }

    pub fn new_maze(width: usize, height: usize, seed: u32) -> Self {
        let mut maze = Self::new_walled(width, height, seed);
        walk_maze(&mut maze);
        maze
    }

    pub fn width(&self) -> usize {
        let mut greatest_row_len = 0;

        for row in &self.cells {
            let row_len = row.len();
            if row_len > greatest_row_len {
                greatest_row_len = row_len;
            }
        }

        greatest_row_len
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn max_x_index(&self) -> usize {
        self.width() - 1
    }

    pub fn max_y_index(&self) -> usize {
        self.height() - 1
    }

    pub fn is_wall_between(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
        let cell1 = match self.clone_at(x1, y1) {
            Some(c) => c,
            None => return false,
        };
        let cell2 = match self.clone_at(x2, y2) {
            Some(c) => c,
            None => return false,
        };

        // same x, with y diff by 1
        if x1 == x2 {
            if (y1 + 1) == y2 {
                return cell1.bottom_wall || cell2.top_wall;
            }
            if y1 as i32 - 1 == y2 as i32 {
                return cell1.top_wall || cell2.bottom_wall;
            }
        }

        // same y, with x diff by 1
        if y1 == y2 {
            if (x1 + 1) == x2 {
                return cell1.right_wall || cell2.left_wall;
            }
            if x1 as i32 - 1 == x2 as i32 {
                return cell1.left_wall || cell2.right_wall;
            }
        }

        false
    }

    pub fn clone_at(&self, x_index: usize, y_index: usize) -> Option<Cell> {
        match self.clone().at(x_index, y_index) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn at(&mut self, x_index: usize, y_index: usize) -> Option<&mut Cell> {
        if x_index > self.max_x_index() || y_index > self.max_y_index() {
            return None;
        }
        if let Some(row) = self.cells.get_mut(y_index) {
            if let Some(cell) = row.get_mut(x_index) {
                return Some(cell);
            }
        }
        None
    }

    pub fn set_top_wall(&mut self, x_index: usize, y_index: usize, top_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.top_wall = top_wall;
        }
    }

    pub fn set_bottom_wall(&mut self, x_index: usize, y_index: usize, bottom_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.bottom_wall = bottom_wall;
        }
    }

    pub fn set_left_wall(&mut self, x_index: usize, y_index: usize, left_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.left_wall = left_wall;
        }
    }

    pub fn set_right_wall(&mut self, x_index: usize, y_index: usize, right_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.right_wall = right_wall;
        }
    }
}

impl Cell {
    fn new() -> Self {
        Cell {
            top_wall: false,
            bottom_wall: false,
            left_wall: false,
            right_wall: false,
        }
    }

    pub fn is_walkable(&self) -> bool {
        // TODO: impliment cells with objects, obstacles, etc. inside of them
        // that are either walkable or unwalkable by the player
        true
    }
}

fn walk_maze(grid: &mut Grid) {
    let mut walked: HashSet<(usize, usize)> = HashSet::new();
    let mut prev_pos_stack: Vec<(usize, usize)> = Vec::new();

    let mut curr_x = 0;
    let mut curr_y = 0;

    let area = grid.width() * grid.height();
    while walked.len() < area {
        let pos = (curr_x, curr_y);
        walked.insert(pos);

        match get_next_pos(&grid, &curr_x, &curr_y, &walked) {
            Some((next_x, next_y)) => {
                // Carve out a passage between the two cells
                if curr_x == next_x {
                    if curr_y < next_y {
                        grid.set_bottom_wall(curr_x, curr_y, false);
                        grid.set_top_wall(next_x, next_y, false);
                    } else {
                        grid.set_top_wall(curr_x, curr_y, false);
                        grid.set_bottom_wall(next_x, next_y, false);
                    }
                } else {
                    if curr_x < next_x {
                        grid.set_right_wall(curr_x, curr_y, false);
                        grid.set_left_wall(next_x, next_y, false);
                    } else {
                        grid.set_left_wall(curr_x, curr_y, false);
                        grid.set_right_wall(next_x, next_y, false);
                    }
                }

                // Add pos to stack
                prev_pos_stack.push((curr_x, curr_y));
                curr_x = next_x;
                curr_y = next_y;
            }
            None => {
                // Backtrack to most recently walked cell
                if let Some((prev_x, prev_y)) = prev_pos_stack.pop() {
                    curr_x = prev_x;
                    curr_y = prev_y;
                } else {
                    // Break if no more cells to backtrack to
                    break;
                }
            }
        }
    }
}

fn get_next_pos(
    grid: &Grid,
    x: &usize,
    y: &usize,
    walked: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    let mut posib_next_pos_list: Vec<(usize, usize)> = vec![];

    if x.to_owned() != 0 {
        posib_next_pos_list.push((x.clone() - 1, y.clone()));
    }
    if x.to_owned() + 1 <= grid.max_x_index() {
        posib_next_pos_list.push((x.clone() + 1, y.clone()));
    }
    if y.to_owned() != 0 {
        posib_next_pos_list.push((x.clone(), y.clone() - 1));
    }
    if y.to_owned() + 1 <= grid.max_y_index() {
        posib_next_pos_list.push((x.clone(), y.clone() + 1));
    }

    posib_next_pos_list.retain(|pos| !walked.contains(pos));

    if posib_next_pos_list.len() == 0 {
        return None;
    }

    // Procedurally generate next pos from number of cells walked & seed
    let index =
        (((grid.seed as usize * walked.len()) as f64).sqrt()) as usize % posib_next_pos_list.len();

    if let Some(pos) = posib_next_pos_list.get(index) {
        return Some(pos.to_owned());
    }

    None
}
