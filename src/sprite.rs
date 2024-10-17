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

#[derive(Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct GroupItem {
    pub position: (i32, i32),
    pub color: (f32, f32, f32, f32),
}

#[derive(Debug)]
pub struct Group {
    pub arr: Vec<GroupItem>,
    pub valid: bool,
}

#[derive(Debug)]
pub struct FillColors {
    pub groups: Vec<Group>,
    pub negative_groups: Vec<Group>,
}

#[derive(Debug, Default)]
pub struct GroupDrawer {
    pub groups: Vec<Group>,
    pub negative_groups: Vec<Group>,
    pub draw_size: usize,
    pub position: (f32, f32),
}

impl GroupDrawer {
    pub fn new() -> Self {
        GroupDrawer { ..default() }
    }

    pub fn _ready() {
        // TODO: ...
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
                if _get_at_pos(map, (pos.0 as i32 + i, pos.1 as i32 + j)) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn _get_at_pos(map: &Vec<Vec<bool>>, pos: (i32, i32)) -> bool {
    if pos.0 < 0
        || pos.0 >= map.len() as i32
        || pos.1 < 0
        || (pos.0 >= 0 && pos.1 >= map[pos.0 as usize].len() as i32)
    {
        return false;
    }

    if pos.0 < 0 || pos.1 < 0 {
        return false;
    }

    map[pos.0 as usize][pos.1 as usize]
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
            arr.push(!_get_at_pos(map, (x as i32, y as i32)))
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
                            if !left || !up || !down || !right {
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
                        if right
                            && pos.0 >= 0
                            && pos.1 >= 0
                            && !checked_map[pos.0 as usize + 1][pos.1 as usize]
                        {
                            bucket.push((pos.0 + 1, pos.1));
                            checked_map[(pos.0 + 1) as usize][pos.1 as usize] = true;
                        }
                        if left
                            && pos.0 - 1 >= 0
                            && pos.1 >= 0
                            && !checked_map[(pos.0 - 1) as usize][pos.1 as usize]
                        {
                            bucket.push((pos.0 - 1, pos.1));
                            checked_map[(pos.0 - 1) as usize][pos.1 as usize] = true;
                        }
                        if down
                            && pos.0 >= 0
                            && pos.1 + 1 >= 0
                            && !checked_map[pos.0 as usize][(pos.1 + 1) as usize]
                        {
                            bucket.push((pos.0, pos.1 + 1));
                            checked_map[pos.0 as usize][(pos.1 + 1) as usize] = true;
                        }
                        if up
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
    right: bool,
    left: bool,
    down: bool,
    up: bool,
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
    if !down {
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
    if !right {
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
    if !up {
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
    if !left {
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
    // actually choose a color
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
