use rand::*;

const BIRTH_LIMIT: u32 = 5;
const DEATH_LIMIT: u32 = 4;
const N_STEPS: u32 = 4;

#[derive(Debug)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct GroupItem {
    pub position: (i32, i32),
    pub color: (f32, f32, f32),
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

pub fn map_generator_generate_new(size: Size) -> Vec<Vec<bool>> {
    let mut map = _get_random_map(&size);
    for i in 0..2 {
        _random_walk(&size, &mut map);
    }
    return map;
}

pub fn _get_random_map(size: &Size) -> Vec<Vec<bool>> {
    let mut map = vec![];
    for x in 0..size.x {
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
    for i in 0..100 {
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
    for i in 0..N_STEPS {
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

    if (pos.0 < 0 || pos.1 < 0) {
        return false;
    }

    map[pos.0 as usize][pos.1 as usize]
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
