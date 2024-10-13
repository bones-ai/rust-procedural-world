use bevy::ui::Val;

use crate::*;

pub fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_W as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_H as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}

pub fn world_to_grid(x: f32, y: f32) -> (f32, f32) {
    (
        (x / (TILE_W as f32 * SPRITE_SCALE_FACTOR as f32)).floor(),
        (y / (TILE_H as f32 * SPRITE_SCALE_FACTOR as f32)).floor(),
    )
}

pub fn center_to_top_left_grid(x: f32, y: f32) -> (f32, f32) {
    let x_center = x + GRID_COLS as f32 / 2.0;
    let y_center = GRID_ROWS as f32 / 2.0 - y;
    (x_center, y_center)
}

pub fn center_to_top_left(x: f32, y: f32) -> (f32, f32) {
    let x_center = x - (GRID_W as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0;
    let y_center = (GRID_H as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0 - y;
    (x_center, y_center)
}

pub fn grid_to_chunk(x: f32, y: f32) -> (i32, i32) {
    let (x, y) = (x / CHUNK_W as f32, y / CHUNK_H as f32);
    (x.floor() as i32, y.floor() as i32)
}

pub fn world_to_chunk(x: f32, y: f32) -> (i32, i32) {
    let (x, y) = world_to_grid(x, y);
    grid_to_chunk(x, y)
}

pub fn seed_from_seed_str(seed_str: String) -> u32 {
    seed_str
        .trim()
        .split("")
        .map(|c| {
            c.as_bytes()
                .iter()
                .map(|i| i.to_owned() as u32)
                .fold(0, |acc, i| acc + i)
        })
        .fold(0, |acc: u32, j: u32| acc.wrapping_add(j))
}

pub fn diff_exceeds_max(n1: f32, n2: f32, max: f32) -> bool {
    let diff = if n1 >= n2 { n1 - n2 } else { n2 - n1 };
    diff > max
}

pub fn add_px_vals(val1: Val, val2: Val) -> Val {
    match (val1, val2) {
        (Val::Px(px1), Val::Px(px2)) => Val::Px(px1 + px2),
        _ => panic!("Both values must be Val::Px"),
    }
}

pub fn px_val_greater_than(val1: Val, val2: Val) -> bool {
    match (val1, val2) {
        (Val::Px(px1), Val::Px(px2)) => px1 > px2,
        _ => false,
    }
}

pub fn px_val_greater_than_or_eq(val1: Val, val2: Val) -> bool {
    match (val1, val2) {
        (Val::Px(px1), Val::Px(px2)) => px1 >= px2,
        _ => false,
    }
}

pub fn px_val_less_than(val1: Val, val2: Val) -> bool {
    match (val1, val2) {
        (Val::Px(px1), Val::Px(px2)) => px1 < px2,
        _ => false,
    }
}

pub fn px_val_less_than_or_eq(val1: Val, val2: Val) -> bool {
    match (val1, val2) {
        (Val::Px(px1), Val::Px(px2)) => px1 <= px2,
        _ => false,
    }
}

pub fn px_val_between(val: Val, min: Val, max: Val) -> bool {
    px_val_greater_than_or_eq(val, min) && px_val_less_than_or_eq(val, max)
}

pub fn proc_gen_num(seed: u32, incr: usize, rem: usize) -> usize {
    (((seed as usize * incr) as f64).sqrt()) as usize % rem
}
