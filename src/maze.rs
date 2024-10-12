use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Debug, Clone, Component)]
pub struct Maze {
    pub cells: Vec<Vec<MazeCell>>,
    seed: u32,
}

#[derive(Debug, Clone, Component)]
pub struct MazeCell {
    pub top_wall: bool,
    pub bottom_wall: bool,
    pub left_wall: bool,
    pub right_wall: bool,
}

impl Maze {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        if height == 0 || width == 0 {
            panic!(
                "maze height and width must be greater than zero. received height: {} and width: {}",
                height, width,
            );
        }

        let mut cells: Vec<Vec<MazeCell>> = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(MazeCell::new());
            }
            cells.push(row)
        }

        let mut maze = Maze { cells, seed };
        maze.walk();
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

    pub fn clone_at(&self, x_index: usize, y_index: usize) -> Option<MazeCell> {
        match self.clone().at(x_index, y_index) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    fn at(&mut self, x_index: usize, y_index: usize) -> Option<&mut MazeCell> {
        if x_index > self.max_x_index() || y_index > self.max_y_index() {
            return None;
        }
        Some(&mut self.cells[y_index][x_index])
    }

    fn set_top_wall(&mut self, x_index: usize, y_index: usize, top_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.top_wall = top_wall;
        }
    }

    fn set_bottom_wall(&mut self, x_index: usize, y_index: usize, bottom_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.bottom_wall = bottom_wall;
        }
    }

    fn set_left_wall(&mut self, x_index: usize, y_index: usize, left_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.left_wall = left_wall;
        }
    }

    fn set_right_wall(&mut self, x_index: usize, y_index: usize, right_wall: bool) {
        if let Some(cell) = self.at(x_index, y_index) {
            cell.right_wall = right_wall;
        }
    }

    fn get_next_pos(
        &self,
        x: &usize,
        y: &usize,
        walked: &HashSet<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let mut posib_next_pos_list: Vec<(usize, usize)> = vec![];

        if x.to_owned() != 0 {
            posib_next_pos_list.push((x.clone() - 1, y.clone()));
        }
        if x.to_owned() + 1 <= self.max_x_index() {
            posib_next_pos_list.push((x.clone() + 1, y.clone()));
        }
        if y.to_owned() != 0 {
            posib_next_pos_list.push((x.clone(), y.clone() - 1));
        }
        if y.to_owned() + 1 <= self.max_y_index() {
            posib_next_pos_list.push((x.clone(), y.clone() + 1));
        }

        posib_next_pos_list.retain(|pos| !walked.contains(pos));

        if posib_next_pos_list.len() == 0 {
            return None;
        }

        // Procedurally generate next pos from number of cells walked & seed
        let index = (((self.seed as usize * walked.len()) as f64).sqrt()) as usize
            % posib_next_pos_list.len();

        if let Some(pos) = posib_next_pos_list.get(index) {
            return Some(pos.to_owned());
        }

        None
    }

    fn walk(&mut self) {
        let mut walked: HashSet<(usize, usize)> = HashSet::new();
        let mut prev_pos_stack: Vec<(usize, usize)> = Vec::new();

        let mut curr_x = 0;
        let mut curr_y = 0;

        let area = self.width() * self.height();
        while walked.len() < area {
            let pos = (curr_x, curr_y);
            walked.insert(pos);

            match self.get_next_pos(&curr_x, &curr_y, &walked) {
                Some((next_x, next_y)) => {
                    // Carve out a passage between the two cells
                    if curr_x == next_x {
                        if curr_y < next_y {
                            self.set_bottom_wall(curr_x, curr_y, false);
                            self.set_top_wall(next_x, next_y, false);
                        } else {
                            self.set_top_wall(curr_x, curr_y, false);
                            self.set_bottom_wall(next_x, next_y, false);
                        }
                    } else {
                        if curr_x < next_x {
                            self.set_right_wall(curr_x, curr_y, false);
                            self.set_left_wall(next_x, next_y, false);
                        } else {
                            self.set_left_wall(curr_x, curr_y, false);
                            self.set_right_wall(next_x, next_y, false);
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
}

impl MazeCell {
    fn new() -> Self {
        MazeCell {
            top_wall: true,
            bottom_wall: true,
            left_wall: true,
            right_wall: true,
        }
    }
}
