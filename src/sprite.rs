use std::fs;

use bevy::prelude::default;
use noise::{NoiseFn, Perlin};
use rand::*;

const BIRTH_LIMIT: u32 = 5;
const DEATH_LIMIT: u32 = 4;
const N_STEPS: u32 = 4;

const SIZE: Size = Size { x: 45, y: 45 };
const DRAW_RECT: Size = Size { x: 400, y: 400 };
const OUTLINE: bool = true;
const SEED: u32 = 1234;

const MOVEMENT: bool = true;
const DRAW_SIZE: usize = 10;

#[derive(Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct GroupItem {
    pub position: (i32, i32),
    pub color: (f32, f32, f32, f32),
}

#[derive(Clone, Debug)]
pub struct Group {
    pub arr: Vec<GroupItem>,
    pub valid: bool,
    pub start_time: usize,
}

#[derive(Debug)]
pub struct FillColors {
    pub groups: Vec<Group>,
    pub negative_groups: Vec<Group>,
}

#[derive(Clone, Debug, Default)]
pub struct GroupDrawer {
    pub groups: Vec<Group>,
    pub negative_groups: Vec<Group>,
    pub draw_size: usize,
    pub position: (f32, f32),
    pub children: Vec<CellDrawer>,
}

#[derive(Clone, Debug, Default)]
pub struct CellDrawer {
    pub cells: Vec<GroupItem>,
    pub lifetime: usize,
    pub movement: bool,
    pub draw_size: usize,
    pub is_eye: bool,
}

impl GroupDrawer {
    pub fn new() -> Self {
        GroupDrawer { ..default() }
    }

    pub fn add_child(&mut self, cell_drawer: CellDrawer) {
        self.children.push(cell_drawer);
    }

    pub fn _ready(&mut self) {
        let mut largest: usize = 0;
        for g in &self.groups {
            largest = max(largest, g.arr.len());
        }

        let group_len = self.groups.len();

        for i in (0..group_len as i32).rev() {
            if i < 0 {
                continue;
            }

            if let Some(g) = self.groups.get_mut(i as usize) {
                g.start_time = g.arr.len() + group_len;
                if g.arr.len() as f32 >= largest as f32 * 0.25 {
                    let mut cell_drawer = CellDrawer::new();
                    cell_drawer.set_cells(g.arr.clone());
                    cell_drawer.lifetime = g.start_time.clone();
                    cell_drawer.movement = MOVEMENT;

                    self.add_child(cell_drawer);
                } else {
                    self.groups.remove(i as usize);
                }
            }
        }

        for i in 0..self.negative_groups.len() {
            let g = &mut self.negative_groups[i];
            if g.valid {
                let mut touching = false;
                for g2 in &mut self.groups {
                    if group_is_touching_group(&g, g2) {
                        touching = true;
                        if g.start_time != 0 {
                            g2.start_time = g.start_time;
                        } else {
                            g.start_time = g2.start_time;
                        }
                    }
                }

                if touching {
                    let mut cell_drawer = CellDrawer::new();
                    cell_drawer.set_cells(g.arr.clone());

                    cell_drawer.lifetime = g.start_time;
                    cell_drawer.movement = MOVEMENT;

                    if (g.arr.len() + self.negative_groups.len()) % 5 >= 3 {
                        cell_drawer.set_eye();
                    }

                    self.add_child(cell_drawer);
                }
            }
        }

        for c in &mut self.children {
            c.draw_size = DRAW_SIZE;
        }
    }

    pub fn draw_all(&self) -> Vec<Vec<(f32, f32, f32, f32)>> {
        let mut board: Vec<Vec<(f32, f32, f32, f32)>> = vec![];
        for _ in 0..SIZE.y {
            let mut row = vec![];
            for _ in 0..SIZE.x {
                // Solid white
                row.push((255.0, 255.0, 255.0, 1.0));
            }
            board.push(row);
        }

        for c in self.children.iter() {
            c._draw(&mut board);
        }
        board
    }

    pub fn write_html_file(&self, html_file_path: &str) {
        let board = self.draw_all();
        let html = html_from_board(board);
        fs::write(html_file_path, html).unwrap();
    }
}

impl CellDrawer {
    pub fn new() -> Self {
        CellDrawer { ..default() }
    }

    pub fn set_cells(&mut self, cells: Vec<GroupItem>) {
        self.cells = cells;
    }

    pub fn set_eye(&mut self) {
        self.is_eye = true;
    }

    pub fn _draw(&self, board: &mut Vec<Vec<(f32, f32, f32, f32)>>) {
        let mut average: (i32, i32) = (0, 0);
        let mut size: f64 = 0.0;
        let mut eye_cutoff: f64 = 0.0;
        if self.is_eye {
            for c in self.cells.iter() {
                size += 1.0;
                average.0 += c.position.0;
                average.1 += c.position.1;
            }
            eye_cutoff = size.sqrt() * 0.3;
        }

        average.0 = average.0 / self.cells.len() as i32;
        average.1 = average.1 / self.cells.len() as i32;

        for c in self.cells.iter() {
            if c.position.0 < 0 && c.position.1 < 0 {
                continue;
            }
            if let Some(row) = board.get(c.position.1 as usize) {
                if let Some(_) = row.get(c.position.0 as usize) {
                    let color = if self.is_eye && dist_between(average, c.position) < eye_cutoff {
                        darken_rgba(c.color, 0.85)
                    } else {
                        c.color
                    };

                    board[c.position.1 as usize][c.position.0 as usize] = color;
                }
            }
        }
    }
}

pub fn _get_group_drawer(pixel_perfect: bool) -> GroupDrawer {
    let sprite_groups = get_sprite(SEED, &SIZE, 12, OUTLINE);
    let mut gd = GroupDrawer::new();
    gd.groups = sprite_groups.groups;
    gd.negative_groups = sprite_groups.negative_groups;

    let draw_size = min(DRAW_RECT.x / SIZE.x, DRAW_RECT.y / SIZE.y);
    if pixel_perfect {
        gd.draw_size = 1;
    } else {
        gd.draw_size = draw_size;
        gd.position = (
            (draw_size * SIZE.x) as f32 * -0.5,
            (draw_size * SIZE.y) as f32 * -0.5,
        );
    }

    gd
}

pub fn get_sprite(seed: u32, size: &Size, n_colors: usize, outline: bool) -> GroupDrawer {
    let mut map = _get_random_map(size);

    map = cellular_automata_do_steps(&mut map);

    let scheme = colorscheme_generator_generate_new_colorscheme(n_colors);
    let eye_scheme = colorscheme_generator_generate_new_colorscheme(n_colors);

    let all_groups = color_filler_fill_colors(&mut map, scheme, eye_scheme, n_colors, outline);

    let mut group_drawer = GroupDrawer::new();
    group_drawer.groups = all_groups.groups;
    group_drawer.negative_groups = all_groups.negative_groups;

    group_drawer
}

pub fn map_generator_generate_new(size: Size) -> Vec<Vec<bool>> {
    let mut map = _get_random_map(&size);
    for _ in 0..2 {
        _random_walk(&size, &mut map);
    }
    return map;
}

pub fn _get_random_map(size: &Size) -> Vec<Vec<bool>> {
    let mut map = vec![];
    for _ in 0..size.x {
        map.push(vec![]);
    }

    for x in 0..(size.x as f32 * 0.5).ceil() as usize {
        let mut arr = vec![];
        for y in 0..size.y {
            arr.push(rand_bool(0.48));

            // When close to center increase the cances to fill the map, so it's more likely to end up with a sprite that's connected in the middle
            let to_center = ((y as f32 - size.y as f32 * 0.5).abs() * 2.0) / size.y as f32;
            if x as f32 == (size.x as f32 * 0.5).floor() - 1.0
                || x as f32 == (size.x as f32 * 0.5) - 2.0
            {
                if rand_range(0.0, 0.4) > to_center {
                    arr[y] = true;
                }
            }
        }

        map[x] = arr.clone();
        map[size.x - x - 1] = arr.clone();
    }

    map
}

fn _random_walk(size: &Size, map: &mut Vec<Vec<bool>>) {
    let mut pos = (randi() % size.x as i32, randi() % size.y as i32);
    for _ in 0..100 {
        _set_at_pos(map, &pos, true);

        _set_at_pos(map, &(size.x as i32 - pos.0 - 1, pos.1), true);

        pos.0 += randi() % 3 - 1;
        pos.1 += randi() % 3 - 1;
    }
}

fn _set_at_pos(map: &mut Vec<Vec<bool>>, pos: &(i32, i32), val: bool) -> bool {
    if pos.0 < 0
        || pos.0 >= map.len() as i32
        || pos.1 < 0
        || (pos.0 >= 0 && pos.1 >= map[pos.0 as usize].len() as i32)
    {
        return false;
    }

    if pos.0 > 0 && pos.1 > 0 {
        map[pos.0 as usize][pos.1 as usize] = val;
    }

    true
}

pub fn cellular_automata_do_steps(map: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut clone = map.clone();
    for _ in 0..N_STEPS {
        clone = _step(&mut clone.clone());
    }
    clone
}

fn _step(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut dup = map.clone();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let cell = dup[x][y];
            let n = _get_neighbours(map, (x, y));
            if cell && n < DEATH_LIMIT {
                dup[x][y] = false;
            } else if !cell && n > BIRTH_LIMIT {
                dup[x][y] = true;
            }
        }
    }
    dup
}

fn _get_neighbours(map: &Vec<Vec<bool>>, pos: (usize, usize)) -> u32 {
    let mut count = 0;

    for i in -1i32..2 {
        for j in -1i32..2 {
            if !(i == 0 && j == 0) {
                if let Some(val) = _get_at_pos(map, (pos.0 as i32 + i, pos.1 as i32 + j)) {
                    if val {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn _get_at_pos(map: &Vec<Vec<bool>>, pos: (i32, i32)) -> Option<bool> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    if pos.0 < 0
        || pos.0 >= map.len() as i32
        || pos.1 < 0
        || (pos.0 >= 0 && pos.1 >= map[pos.0 as usize].len() as i32)
    {
        return Some(false);
    }

    Some(map[pos.0 as usize][pos.1 as usize])
}

pub fn colorscheme_generator_generate_new_colorscheme(
    n_colors: usize,
) -> Vec<(f32, f32, f32, f32)> {
    let a = (
        rand_range(0.0, 0.5),
        rand_range(0.0, 0.5),
        rand_range(0.0, 0.5),
    );

    let b = (
        rand_range(0.1, 0.6),
        rand_range(0.1, 0.6),
        rand_range(0.1, 0.6),
    );

    let c = (
        rand_range(0.15, 0.8),
        rand_range(0.15, 0.8),
        rand_range(0.15, 0.8),
    );

    let d = (
        rand_range(0.0, 1.0),
        rand_range(0.0, 1.0),
        rand_range(0.0, 1.0),
    );

    let mut cols = vec![];
    let n = (n_colors - 1) as f32;

    for i in 0..n_colors {
        let vec3 = (
            // r
            (a.0 + b.0 * (6.28318 * (c.0 * (i as f32 / n) + d.0)).cos()) + (i as f32 / n) * 0.8,
            // g
            (a.1 + b.1 * (6.28318 * (c.1 * (i as f32 / n) + d.1)).cos()) + (i as f32 / n) * 0.8,
            // b
            (a.2 + b.2 * (6.28318 * (c.2 * (i as f32 / n) + d.2)).cos()) + (i as f32 / n) * 0.8,
            // a
            1.0,
        );

        cols.push(vec3);
    }

    cols
}

pub fn color_filler_fill_colors(
    map: &mut Vec<Vec<bool>>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    n_colors: usize,
    outline: bool,
) -> FillColors {
    let noise1 = Perlin::new(randu());
    let noise2 = Perlin::new(randu());

    let groups = _flood_fill(
        map,
        colorscheme.clone(),
        eye_colorscheme.clone(),
        n_colors,
        false,
        outline,
        noise1,
        noise2,
    );

    let negative_groups = _flood_fill_negative(
        map,
        colorscheme,
        eye_colorscheme,
        n_colors,
        outline,
        noise1,
        noise2,
    );

    FillColors {
        groups,
        negative_groups,
    }
}

fn _flood_fill_negative(
    map: &mut Vec<Vec<bool>>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    n_colors: usize,
    outline: bool,
    noise1: Perlin,
    noise2: Perlin,
) -> Vec<Group> {
    let mut negative_map = vec![];
    for x in 0..map.len() {
        let mut arr = vec![];
        for y in 0..map[x].len() {
            if let Some(val) = _get_at_pos(map, (x as i32, y as i32)) {
                arr.push(!val);
            }
        }
        negative_map.push(arr);
    }
    return _flood_fill(
        &mut negative_map,
        colorscheme,
        eye_colorscheme,
        n_colors,
        true,
        outline,
        noise1,
        noise2,
    );
}

fn _flood_fill(
    map: &mut Vec<Vec<bool>>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    n_colors: usize,
    is_negative: bool,
    outline: bool,
    noise1: Perlin,
    noise2: Perlin,
) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    let mut checked_map = vec![];
    for x in 0..map.len() {
        let mut arr = vec![];
        for _y in 0..map[x].len() {
            arr.push(false);
        }
        checked_map.push(arr)
    }
    // bucket is all the cells that have been found through flood filling and whose neighbours will be checked next
    let mut bucket: Vec<(i32, i32)> = vec![];
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            // haven't checked this cell yet
            if !checked_map[x][y] {
                checked_map[x][y] = true;
                // if this cell is actually filled in the map
                if map[x][y] {
                    bucket.push((x as i32, y as i32));
                    let mut group = Group {
                        arr: vec![],
                        valid: true,
                        start_time: 0,
                    };
                    // go through remaining cells in bucket
                    while bucket.len() > 0 {
                        let pos: (i32, i32) = match bucket.pop() {
                            None => break,
                            Some(p) => p,
                        };
                        // get neighbours
                        let right = _get_at_pos(map, (pos.0 + 1, pos.1));
                        let left = _get_at_pos(map, (pos.0 - 1, pos.1));
                        let down = _get_at_pos(map, (pos.0, pos.1 + 1));
                        let up = _get_at_pos(map, (pos.0, pos.1 - 1));
                        // dont want negative groups that touch the edge of the sprite
                        if is_negative {
                            if left.is_none() || up.is_none() || down.is_none() || right.is_none() {
                                group.valid = false;
                            }
                        }
                        // also do a coloring step in this flood fill, speeds up processing a bit instead of doing it seperately
                        let col = _get_color(
                            map,
                            pos,
                            is_negative,
                            right,
                            left,
                            down,
                            up,
                            colorscheme.clone(),
                            eye_colorscheme.clone(),
                            n_colors,
                            outline,
                            &mut group,
                            noise1,
                            noise2,
                        );
                        group.arr.push(GroupItem {
                            position: pos,
                            color: col,
                        });
                        // add neighbours to bucket to check
                        if right.is_some()
                            && right.unwrap()
                            && pos.0 >= 0
                            && pos.1 >= 0
                            && !checked_map[pos.0 as usize + 1][pos.1 as usize]
                        {
                            bucket.push((pos.0 + 1, pos.1));
                            checked_map[(pos.0 + 1) as usize][pos.1 as usize] = true;
                        }
                        if left.is_some()
                            && left.unwrap()
                            && pos.0 - 1 >= 0
                            && pos.1 >= 0
                            && !checked_map[(pos.0 - 1) as usize][pos.1 as usize]
                        {
                            bucket.push((pos.0 - 1, pos.1));
                            checked_map[(pos.0 - 1) as usize][pos.1 as usize] = true;
                        }
                        if down.is_some()
                            && down.unwrap()
                            && pos.0 >= 0
                            && pos.1 + 1 >= 0
                            && !checked_map[pos.0 as usize][(pos.1 + 1) as usize]
                        {
                            bucket.push((pos.0, pos.1 + 1));
                            checked_map[pos.0 as usize][(pos.1 + 1) as usize] = true;
                        }
                        if up.is_some()
                            && up.unwrap()
                            && pos.0 >= 0
                            && pos.1 - 1 >= 0
                            && !checked_map[pos.0 as usize][(pos.1 - 1) as usize]
                        {
                            bucket.push((pos.0, pos.1 - 1));
                            checked_map[pos.0 as usize][(pos.1 - 1) as usize] = true;
                        }
                    }
                    groups.push(group)
                }
            }
        }
    }
    groups
}

fn _get_color(
    map: &mut Vec<Vec<bool>>,
    pos: (i32, i32),
    is_negative: bool,
    right: Option<bool>,
    left: Option<bool>,
    down: Option<bool>,
    up: Option<bool>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    n_colors: usize,
    outline: bool,
    group: &mut Group,
    noise1: Perlin,
    noise2: Perlin,
) -> (f32, f32, f32, f32) {
    let col_x = (pos.0 as f64 - (map.len() - 1) as f64 * 0.5).abs().ceil();
    let mut n1 = (noise1.get([col_x, pos.1 as f64])).abs().powf(1.5) * 3.0;
    let mut n2 = (noise2.get([col_x, pos.1 as f64])).abs().powf(1.5) * 3.0;

    // highlight colors based on amount of neighbours
    if down.is_none() || !down.unwrap() {
        if is_negative {
            n2 -= 0.1;
        } else {
            n1 -= 0.45;
            n1 *= 0.8;
        }
        if outline {
            group.arr.push(GroupItem {
                position: (pos.0, pos.1 + 1),
                color: (0.0, 0.0, 0.0, 1.0),
            });
        }
    }
    if right.is_none() || !right.unwrap() {
        if is_negative {
            n2 += 0.1;
        } else {
            n1 += 0.2;
            n1 *= 1.1;
        }
        if outline {
            group.arr.push(GroupItem {
                position: (pos.0 + 1, pos.1),
                color: (0.0, 0.0, 0.0, 1.0),
            });
        }
    }
    if up.is_none() || !up.unwrap() {
        if is_negative {
            n2 += 0.15;
        } else {
            n1 += 0.45;
            n1 *= 1.2;
        }
        if outline {
            group.arr.push(GroupItem {
                position: (pos.0, pos.1 - 1),
                color: (0.0, 0.0, 0.0, 1.0),
            });
        }
    }
    if left.is_none() || !left.unwrap() {
        if is_negative {
            n2 += 0.1;
        } else {
            n1 += 0.2;
            n1 *= 1.1;
        }
        if outline {
            group.arr.push(GroupItem {
                position: (pos.0 - 1, pos.1),
                color: (0.0, 0.0, 0.0, 1.0),
            });
        }
    }
    // highlight colors if the difference in colors between neighbours is big
    let c_0 =
        colorscheme[(noise1.get([col_x, pos.1 as f64]) * (n_colors as f64 - 1.0)).floor() as usize];
    let c_1 = colorscheme
        [(noise1.get([col_x, (pos.1 - 1) as f64]) * (n_colors as f64 - 1.0)).floor() as usize];
    let c_2 = colorscheme
        [(noise1.get([col_x, (pos.1 + 1) as f64]) * (n_colors as f64 - 1.0)).floor() as usize];
    let c_3 = colorscheme
        [(noise1.get([col_x - 1.0, pos.1 as f64]) * (n_colors as f64 - 1.0)).floor() as usize];
    let c_4 = colorscheme
        [(noise1.get([col_x + 1.0, pos.1 as f64]) * (n_colors as f64 - 1.0)).floor() as usize];
    let diff = ((c_0.0 - c_1.0).abs() + (c_0.1 - c_1.1).abs() + (c_0.2 - c_1.2).abs())
        + ((c_0.0 - c_2.0).abs() + (c_0.1 - c_2.1.abs()) + (c_0.2 - c_2.2).abs())
        + ((c_0.0 - c_3.0).abs() + (c_0.1 - c_3.1).abs() + (c_0.2 - c_3.2).abs())
        + ((c_0.0 - c_4.0).abs() + (c_0.1 - c_4.1).abs() + (c_0.2 - c_4.2).abs());
    if diff > 2.0 {
        n1 += 0.3;
        n1 *= 1.5;
        n2 += 0.3;
        n2 *= 1.5;
    }

    // choose a color
    n1 = clamp(n1, 0.0, 1.0);
    n1 = (n1 * (n_colors as f64 - 1.0)).floor();
    n2 = clamp(n2, 0.0, 1.0);
    n2 = (n2 * (n_colors as f64 - 1.0)).floor();
    let mut col = colorscheme[n1 as usize];
    if is_negative {
        col = eye_colorscheme[n2 as usize];
    }
    col
}

fn group_is_touching_group(g1: &Group, g2: &Group) -> bool {
    for c in &g1.arr {
        for c2 in &g2.arr {
            if c.position.0 == c2.position.0 {
                if c.position.1 == c2.position.1 + 1 || c.position.1 == c2.position.1 - 1 {
                    return true;
                }
            } else if c.position.1 == c2.position.1 {
                if c.position.0 == c2.position.0 + 1 || c.position.0 == c2.position.0 - 1 {
                    return true;
                }
            }
        }
    }

    false
}

fn rand_bool(chance: f32) -> bool {
    rand_range(0.0, 1.0) > chance
}

fn rand_range(n1: f32, n2: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(n1..n2)
}

fn randi() -> i32 {
    rand::thread_rng().gen()
}

fn randu() -> u32 {
    rand::thread_rng().gen_range(0..u32::MAX)
}

fn clamp(n: f64, min: f64, max: f64) -> f64 {
    if n > max {
        return max;
    }
    if n < min {
        return min;
    }
    n
}

fn min(n1: usize, n2: usize) -> usize {
    if n1 > n2 {
        return n2;
    }
    n1
}

fn max(n1: usize, n2: usize) -> usize {
    if n1 < n2 {
        return n2;
    }
    n1
}

fn dist_between(t1: (i32, i32), t2: (i32, i32)) -> f64 {
    let dx = (t2.0 - t1.0) as f64;
    let dy = (t2.1 - t1.1) as f64;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

fn rgba_to_hex(rgba: (f32, f32, f32, f32)) -> String {
    let (r, g, b, a) = rgba;

    // Convert to u8
    let r = (r * 255.0).round() as u8;
    let g = (g * 255.0).round() as u8;
    let b = (b * 255.0).round() as u8;
    let a = (a * 255.0).round() as u8;

    // Create hex string
    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

fn darken_rgba((r, g, b, a): (f32, f32, f32, f32), perc: f32) -> (f32, f32, f32, f32) {
    let perc = perc.clamp(0.0, 1.0);

    let darkened_r = r * (1.0 - perc);
    let darkened_g = g * (1.0 - perc);
    let darkened_b = b * (1.0 - perc);

    (darkened_r, darkened_g, darkened_b, a)
}

fn html_from_board(board: Vec<Vec<(f32, f32, f32, f32)>>) -> String {
    let mut board_inner_html = vec![];

    for row in board {
        for (r, g, b, a) in row {
            let div = format!(
                r#"
                    <div style="height:8px; width:8px; background-color:{}; border:solid black 1px;"></div>
                "#,
                rgba_to_hex((r, g, b, a)),
            );

            board_inner_html.push(div.trim().to_owned());
        }
    }

    format!(
        r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Sprite</title>
            </head>
            <body>
                <div style="display: inline-grid; grid-template-columns:repeat({}, 1fr)">
                    {}
                </div>
            </body>
            </html>
        "#,
        SIZE.x,
        board_inner_html.join(""),
    )
}
