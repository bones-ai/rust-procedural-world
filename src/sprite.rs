use std::{
    collections::HashSet,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::prelude::{default, Color};
use noise::{NoiseFn, Perlin};
use rand::{rngs::StdRng, Rng};

use crate::utils::seed_to_rng;

const BIRTH_LIMIT: u32 = 5;
const DEATH_LIMIT: u32 = 4;
const N_STEPS: u32 = 4;
const N_COLORS: usize = 12;

const SPRITE_HEIGHT: usize = 45;
const SPRITE_WIDTH: usize = 45;
const CELL_HEIGHT_PX: usize = 8;
const CELL_WIDTH_PX: usize = 8;
const PERLIN_SCALE: f64 = 320.5;

#[derive(Debug)]
pub enum Faction {
    ChaosWarriors,
    WaterBoys,
    ForestBoys,
    TechBoys,
    HellSpawn,
    SpaceAliens,
    GoldenBoys,
    JusticeSoldiers,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub position: (i32, i32),
    pub color: (f32, f32, f32, f32),
}

#[derive(Clone, Debug)]
pub struct Component {
    pub cells: Vec<Cell>,
}

#[derive(Clone, Debug)]
pub struct ComponentGroup {
    components: Vec<Component>,
}

pub struct Sprite {
    component_groups: Vec<ComponentGroup>,
}

#[derive(Clone, Debug, Default)]
pub struct ComponentDrawer {
    pub component_groups: Vec<ComponentGroup>,
    pub neg_components: Vec<Component>,
    pub components: Vec<Component>,
}

impl Component {
    pub fn _draw(&self, board: &mut Vec<Vec<(f32, f32, f32, f32)>>) {
        for c in self.cells.iter() {
            if c.position.0 < 0 && c.position.1 < 0 {
                continue;
            }
            if let Some(row) = board.get(c.position.1 as usize) {
                if let Some(_) = row.get(c.position.0 as usize) {
                    board[c.position.1 as usize][c.position.0 as usize] = c.color;
                }
            }
        }
    }
}

impl Sprite {
    pub fn new(seed: u32) -> Self {
        let mut cd = get_sprite(seed, 45, 45);
        cd.ready();
        cd.draw_all();

        Sprite {
            component_groups: group_components(cd.components),
        }
    }

    pub fn new_from_unix_seed() -> Self {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).unwrap();
        let unix_timestamp = duration.as_micros();
        Self::new(unix_timestamp as u32)
    }

    pub fn write_html_file(&self, html_file_path: &str) {
        let mut board_inner_html = vec![];

        for component_group in self.component_groups.iter() {
            for i in 0..component_group.components.len() {
                let c = component_group.components[i].clone();

                let mut group = vec![];
                for cell in c.cells.iter() {
                    let (r, g, b, a) = cell.color;
                    let top_px = cell.position.1 * CELL_HEIGHT_PX as i32;
                    let left_px = cell.position.0 * CELL_WIDTH_PX as i32;

                    let div = format!(
                        r#"
                            <div style="position:absolute; top:{}px; left:{}px; height:8px; width:8px; background-color:{};"></div>
                        "#,
                        top_px,
                        left_px,
                        rgba_to_hex((r, g, b, a)),
                    );

                    group.push(div);
                }

                let group_div = format!(
                    r#"
                        <div style="position:absolute" class="group" data-group="{}">
                            <div style="position:relative;">
                                {}
                            </div>
                        </div>
                    "#,
                    i,
                    group.join(""),
                );

                board_inner_html.push(group_div);
            }
        }

        let html = format!(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Sprite</title>
                </head>
                <body>
                    <script defer src="/sprite_movement.js"></script>
                    <div id="board" style="position:relative; height:{}px; width:{}px">
                        {}
                    </div>
                </body>
                </html>
            "#,
            SPRITE_HEIGHT * CELL_HEIGHT_PX,
            SPRITE_WIDTH * CELL_WIDTH_PX,
            board_inner_html.join(""),
        );

        fs::write(html_file_path, html.trim()).unwrap();
    }
}

impl ComponentDrawer {
    pub fn new(component_groups: Vec<ComponentGroup>, neg_components: Vec<Component>) -> Self {
        ComponentDrawer {
            component_groups,
            neg_components,
            ..default()
        }
    }

    pub fn add_child(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn get_primary_color(&self) -> Color {
        let mut sum_r: f32 = 0.0;
        let mut sum_g: f32 = 0.0;
        let mut sum_b: f32 = 0.0;

        for c in self.components.iter() {
            for cell in c.cells.iter() {
                let (r, g, b, _) = cell.color;
                sum_r += r;
                sum_g += g;
                sum_b += b;
            }
        }

        if sum_r >= sum_b && sum_r >= sum_b {
            return Color::RED;
        }
        if sum_g >= sum_r && sum_g >= sum_b {
            return Color::GREEN;
        }
        if sum_b >= sum_r && sum_b >= sum_g {
            return Color::BLUE;
        }

        Color::RED
    }

    pub fn get_faction(&self) -> Faction {
        let mut sum_r: f32 = 0.0;
        let mut sum_g: f32 = 0.0;
        let mut sum_b: f32 = 0.0;
        let mut count: f32 = 0.0;

        for c in self.components.iter() {
            for cell in c.cells.iter() {
                let (r, g, b, _) = cell.color;

                if (r == 0.0 && g == 0.0 && b == 0.0) || (r == 255.0 && g == 255.0 && b == 255.0) {
                    continue;
                }

                sum_r += (r * 255.0).round();
                sum_g += (g * 255.0).round();
                sum_b += (b * 255.0).round();
                count += 1.0;
            }
        }

        let avg_r = sum_r / count;
        let avg_g = sum_g / count;
        let avg_b = sum_b / count;

        if avg_r >= 0.0
            && avg_r <= 127.0
            && avg_g >= 0.0
            && avg_g <= 127.0
            && avg_b >= 0.0
            && avg_b <= 127.0
        {
            return Faction::ChaosWarriors;
        }
        if avg_r >= 128.0
            && avg_r <= 255.0
            && avg_g >= 0.0
            && avg_g <= 127.0
            && avg_b >= 0.0
            && avg_b <= 127.0
        {
            return Faction::WaterBoys;
        }
        if avg_r >= 0.0
            && avg_r <= 127.0
            && avg_g >= 128.0
            && avg_g <= 255.0
            && avg_b >= 0.0
            && avg_b <= 127.0
        {
            return Faction::ForestBoys;
        }
        if avg_r >= 128.0
            && avg_r <= 255.0
            && avg_g >= 128.0
            && avg_g <= 255.0
            && avg_b >= 0.0
            && avg_b <= 127.0
        {
            return Faction::TechBoys;
        }
        if avg_r >= 0.0
            && avg_r <= 127.0
            && avg_g >= 0.0
            && avg_g <= 127.0
            && avg_b >= 128.0
            && avg_b <= 255.0
        {
            return Faction::HellSpawn;
        }
        if avg_r >= 128.0
            && avg_r <= 255.0
            && avg_g >= 0.0
            && avg_g <= 127.0
            && avg_b >= 128.0
            && avg_b <= 255.0
        {
            return Faction::SpaceAliens;
        }
        if avg_r >= 0.0
            && avg_r <= 127.0
            && avg_g >= 128.0
            && avg_g <= 255.0
            && avg_b >= 128.0
            && avg_b <= 255.0
        {
            return Faction::GoldenBoys;
        }
        if avg_r >= 128.0
            && avg_r <= 255.0
            && avg_g >= 128.0
            && avg_g <= 255.0
            && avg_b >= 128.0
            && avg_b <= 255.0
        {
            return Faction::JusticeSoldiers;
        }

        Faction::ChaosWarriors
    }

    pub fn ready(&mut self) {
        let mut largest: usize = 0;
        for component_group in self.component_groups.iter() {
            for component in component_group.components.iter() {
                largest = max(largest, component.cells.len());
            }
        }

        let component_groups_len = self.component_groups.len();
        let mut children = vec![];

        for i in (0..component_groups_len as i32).rev() {
            if let Some(component_group) = self.component_groups.get(i as usize) {
                for component in component_group.components.iter() {
                    if !(component.cells.len() as f32 >= largest as f32 * 0.25) {
                        self.component_groups.remove(i as usize);
                        break;
                    }

                    let mut dupe_cells: Vec<Cell> = component.cells.clone();

                    for neg_component in self.neg_components.iter_mut() {
                        if components_are_touching(&neg_component, &component) {
                            // Overlay neg_component cells ontop of component cells
                            dupe_cells.append(&mut neg_component.cells);
                        }
                    }

                    children.push(Component { cells: dupe_cells });
                }
            }
        }

        for component in children {
            self.add_child(component);
        }
    }

    pub fn draw_all(&self) -> Vec<Vec<(f32, f32, f32, f32)>> {
        let mut board: Vec<Vec<(f32, f32, f32, f32)>> = vec![];
        for _ in 0..SPRITE_HEIGHT {
            let mut row = vec![];
            for _ in 0..SPRITE_WIDTH {
                // Solid white
                row.push((255.0, 255.0, 255.0, 1.0));
            }
            board.push(row);
        }

        for c in self.components.iter() {
            c._draw(&mut board);
        }

        board
    }
}

fn group_components(components: Vec<Component>) -> Vec<ComponentGroup> {
    let mut component_groups = vec![];
    let mut used_indeces: HashSet<usize> = HashSet::new();

    for i in 0..components.len() {
        if used_indeces.contains(&i) {
            continue;
        }
        used_indeces.insert(i);

        let cp = components[i].clone();
        let mut cp_group = ComponentGroup {
            components: vec![cp.clone()],
        };

        'f: for j in 0..components.len() {
            if j == i || used_indeces.contains(&j) {
                continue 'f;
            }

            let cp2 = &components[j];

            // match mirrored components into component groups of 2
            for cell in cp.cells.iter() {
                let (x, y) = cell.position;
                let pos_reflected = (SPRITE_WIDTH as i32 - 1 - x, y);
                if !cp2.cells.iter().any(|c| c.position == pos_reflected) {
                    continue 'f;
                }
            }

            cp_group.components.push(cp2.clone());
            used_indeces.insert(j);
            break 'f;
        }

        component_groups.push(cp_group);
    }

    component_groups
}

pub fn get_sprite(seed: u32, height: usize, width: usize) -> ComponentDrawer {
    let mut map = make_rand_map(seed, height, width);

    cellular_automata_do_steps(&mut map);

    let (components, neg_components) = fill_colors(seed, &mut map);

    let component_groups = group_components(components);

    ComponentDrawer::new(component_groups, neg_components)
}

pub fn make_rand_map(seed: u32, height: usize, width: usize) -> Vec<Vec<bool>> {
    let mut map = vec![];
    for _ in 0..width {
        map.push(vec![]);
    }

    let mut rng = seed_to_rng(seed);

    for x in 0..(width as f32 * 0.5).ceil() as usize {
        let mut arr = vec![];
        for y in 0..height {
            arr.push(rand_bool(&mut rng, 0.48));

            // When close to center increase the cances to fill the map, so it's more likely to end up with a sprite that's connected in the middle
            let to_center = ((y as f32 - height as f32 * 0.5).abs() * 2.0) / height as f32;
            if x as f32 == (width as f32 * 0.5).floor() - 1.0
                || x as f32 == (width as f32 * 0.5) - 2.0
            {
                if rand_range(&mut rng, 0.0, 0.4) > to_center {
                    arr[y] = true;
                }
            }
        }

        map[x] = arr.clone();
        map[width - x - 1] = arr.clone();
    }

    map
}

pub fn cellular_automata_do_steps(map: &mut Vec<Vec<bool>>) {
    let mut dupe = map.clone();
    for _ in 0..N_STEPS {
        dupe = step(&mut dupe.clone());
    }
    *map = dupe;
}

fn step(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut dup = map.clone();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            // Ensure padding of 1 to prevent overflow when border is added later
            if x == 0 || x == map.len() - 1 || y == 0 || y == map[x].len() - 1 {
                dup[x][y] = false;
                continue;
            }

            let cell = dup[x][y];
            let n = get_neighbours(map, (x, y));
            if cell && n < DEATH_LIMIT {
                dup[x][y] = false;
            } else if !cell && n > BIRTH_LIMIT {
                dup[x][y] = true;
            }
        }
    }
    dup
}

fn get_neighbours(map: &Vec<Vec<bool>>, pos: (usize, usize)) -> u32 {
    let mut count = 0;

    for i in -1i32..2 {
        for j in -1i32..2 {
            if i == 0 && j == 0 {
                continue;
            }

            match get_at_pos(map, (pos.0 as i32 + i, pos.1 as i32 + j)) {
                None => continue,
                Some(val) => {
                    if val {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn get_at_pos(map: &Vec<Vec<bool>>, pos: (i32, i32)) -> Option<bool> {
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

pub fn gen_colorscheme(seed: u32) -> Vec<(f32, f32, f32, f32)> {
    let mut rng = seed_to_rng(seed);

    let a = (
        rand_range(&mut rng, 0.0, 0.5),
        rand_range(&mut rng, 0.0, 0.5),
        rand_range(&mut rng, 0.0, 0.5),
    );

    let b = (
        rand_range(&mut rng, 0.1, 0.6),
        rand_range(&mut rng, 0.1, 0.6),
        rand_range(&mut rng, 0.1, 0.6),
    );

    let c = (
        rand_range(&mut rng, 0.15, 0.8),
        rand_range(&mut rng, 0.15, 0.8),
        rand_range(&mut rng, 0.15, 0.8),
    );

    let d = (
        rand_range(&mut rng, 0.0, 1.0),
        rand_range(&mut rng, 0.0, 1.0),
        rand_range(&mut rng, 0.0, 1.0),
    );

    let mut cols = vec![];
    let n = (N_COLORS - 1) as f32;

    for i in 0..N_COLORS {
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

pub fn fill_colors(seed: u32, map: &mut Vec<Vec<bool>>) -> (Vec<Component>, Vec<Component>) {
    let colorscheme = gen_colorscheme(seed);
    let eye_colorscheme = gen_colorscheme(seed + 1);

    let noise1 = Perlin::new(seed);
    let noise2 = Perlin::new(seed + 1);

    let components = flood_fill(
        map,
        colorscheme.clone(),
        eye_colorscheme.clone(),
        false,
        noise1,
        noise2,
    );

    let neg_components = flood_fill_negative(map, colorscheme, eye_colorscheme, noise1, noise2);

    (components, neg_components)
}

fn flood_fill_negative(
    map: &mut Vec<Vec<bool>>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    noise1: Perlin,
    noise2: Perlin,
) -> Vec<Component> {
    let mut negative_map = vec![];
    for x in 0..map.len() {
        let mut arr = vec![];
        for y in 0..map[x].len() {
            if let Some(val) = get_at_pos(map, (x as i32, y as i32)) {
                arr.push(!val);
            }
        }
        negative_map.push(arr);
    }

    flood_fill(
        &mut negative_map,
        colorscheme,
        eye_colorscheme,
        true,
        noise1,
        noise2,
    )
}

fn flood_fill(
    map: &mut Vec<Vec<bool>>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    is_neg_component: bool,
    noise1: Perlin,
    noise2: Perlin,
) -> Vec<Component> {
    let mut components: Vec<Component> = vec![];
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
                    let mut component = Component { cells: vec![] };
                    let mut valid = true;

                    // go through remaining cells in bucket
                    while bucket.len() > 0 {
                        let pos: (i32, i32) = match bucket.pop() {
                            None => break,
                            Some(p) => p,
                        };
                        // get neighbours
                        let right = get_at_pos(map, (pos.0 + 1, pos.1));
                        let left = get_at_pos(map, (pos.0 - 1, pos.1));
                        let down = get_at_pos(map, (pos.0, pos.1 + 1));
                        let up = get_at_pos(map, (pos.0, pos.1 - 1));
                        // dont want negative groups that touch the edge of the sprite
                        if is_neg_component {
                            if left.is_none() || up.is_none() || down.is_none() || right.is_none() {
                                valid = false;
                            }
                        }
                        // also do a coloring step in this flood fill, speeds up processing a bit instead of doing it seperately
                        let col = choose_color(
                            map,
                            pos,
                            is_neg_component,
                            right,
                            left,
                            down,
                            up,
                            colorscheme.clone(),
                            eye_colorscheme.clone(),
                            &mut component,
                            noise1,
                            noise2,
                        );
                        component.cells.push(Cell {
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

                    if valid {
                        components.push(component)
                    }
                }
            }
        }
    }

    components
}

fn choose_color(
    map: &mut Vec<Vec<bool>>,
    pos: (i32, i32),
    is_neg_component: bool,
    right: Option<bool>,
    left: Option<bool>,
    down: Option<bool>,
    up: Option<bool>,
    colorscheme: Vec<(f32, f32, f32, f32)>,
    eye_colorscheme: Vec<(f32, f32, f32, f32)>,
    component: &mut Component,
    noise1: Perlin,
    noise2: Perlin,
) -> (f32, f32, f32, f32) {
    let col_x = (pos.0 as f64 - (map.len() - 1) as f64 * 0.5).abs().ceil();
    let mut n1 = (noise1.get([col_x / PERLIN_SCALE, pos.1 as f64 / PERLIN_SCALE]))
        .abs()
        .powf(1.5)
        * 3.0;
    let mut n2 = (noise2.get([col_x / PERLIN_SCALE, pos.1 as f64 / PERLIN_SCALE]))
        .abs()
        .powf(1.5)
        * 3.0;

    // highlight colors based on amount of neighbours
    if down.is_none() || !down.unwrap() {
        if is_neg_component {
            n2 -= 0.1;
        } else {
            n1 -= 0.45;
        }
        n1 *= 0.8;
        component.cells.push(Cell {
            position: (pos.0, pos.1 + 1),
            color: (0.0, 0.0, 0.0, 1.0),
        });
    }
    if right.is_none() || !right.unwrap() {
        if is_neg_component {
            n2 += 0.1;
        } else {
            n1 += 0.2;
        }
        n1 *= 1.1;
        component.cells.push(Cell {
            position: (pos.0 + 1, pos.1),
            color: (0.0, 0.0, 0.0, 1.0),
        });
    }
    if up.is_none() || !up.unwrap() {
        if is_neg_component {
            n2 += 0.15;
        } else {
            n1 += 0.45;
        }
        n1 *= 1.2;
        component.cells.push(Cell {
            position: (pos.0, pos.1 - 1),
            color: (0.0, 0.0, 0.0, 1.0),
        });
    }
    if left.is_none() || !left.unwrap() {
        if is_neg_component {
            n2 += 0.1;
        } else {
            n1 += 0.2;
        }
        n1 *= 1.1;
        component.cells.push(Cell {
            position: (pos.0 - 1, pos.1),
            color: (0.0, 0.0, 0.0, 1.0),
        });
    }
    // highlight colors if the difference in colors between neighbours is big
    let c_0 = colorscheme
        [noise1.get([col_x / PERLIN_SCALE, pos.1 as f64 / PERLIN_SCALE]) as usize * (N_COLORS - 1)];
    let c_1 = colorscheme[noise1.get([col_x / PERLIN_SCALE, (pos.1 - 1) as f64 / PERLIN_SCALE])
        as usize
        * (N_COLORS - 1)];
    let c_2 = colorscheme[noise1.get([col_x / PERLIN_SCALE, (pos.1 + 1) as f64 / PERLIN_SCALE])
        as usize
        * (N_COLORS - 1)];
    let c_3 = colorscheme[noise1.get([(col_x - 1.0) / PERLIN_SCALE, pos.1 as f64 / PERLIN_SCALE])
        as usize
        * (N_COLORS - 1)];
    let c_4 = colorscheme[noise1.get([(col_x + 1.0) / PERLIN_SCALE, pos.1 as f64 / PERLIN_SCALE])
        as usize
        * (N_COLORS - 1)];
    let diff = ((c_0.0 - c_1.0).abs() + (c_0.1 - c_1.1).abs() + (c_0.2 - c_1.2).abs())
        + ((c_0.0 - c_2.0).abs() + (c_0.1 - c_2.1).abs() + (c_0.2 - c_2.2).abs())
        + ((c_0.0 - c_3.0).abs() + (c_0.1 - c_3.1).abs() + (c_0.2 - c_3.2).abs())
        + ((c_0.0 - c_4.0).abs() + (c_0.1 - c_4.1).abs() + (c_0.2 - c_4.2).abs());

    if diff > 2.0 {
        n1 += 0.3;
        n1 *= 1.5;
        n2 += 0.3;
        n2 *= 1.5;
    }

    // choose a color
    n1 = n1.clamp(0.0, 1.0);
    n1 = (n1 * (N_COLORS as f64 - 1.0)).floor();
    n2 = n2.clamp(0.0, 1.0);
    n2 = (n2 * (N_COLORS as f64 - 1.0)).floor();
    let mut col = colorscheme[n1 as usize];
    if is_neg_component {
        col = eye_colorscheme[n2 as usize];
    }
    col
}

fn components_are_touching(cp1: &Component, cp2: &Component) -> bool {
    let cp1_positions: Vec<(i32, i32)> = cp1.cells.iter().map(|c: &Cell| c.position).collect();
    let cp2_positions: Vec<(i32, i32)> = cp2.cells.iter().map(|c: &Cell| c.position).collect();

    for &(x, y) in &cp1_positions {
        if cp2_positions.contains(&(x, y)) {
            return true;
        }
    }

    for &(x, y) in &cp2_positions {
        if cp1_positions.contains(&(x, y)) {
            return true;
        }
    }

    false
}

fn rand_bool(rng: &mut StdRng, chance: f32) -> bool {
    rand_range(rng, 0.0, 1.0) > chance
}

fn rand_range(rng: &mut StdRng, n1: f32, n2: f32) -> f32 {
    rng.gen_range(n1..n2)
}

fn max(n1: usize, n2: usize) -> usize {
    if n1 < n2 {
        return n2;
    }
    n1
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
