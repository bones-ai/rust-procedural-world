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

pub fn diff_exceeds_max(n1: f32, n2: f32, max: f32) -> bool {
    let diff = if n1 >= n2 { n1 - n2 } else { n2 - n1 };
    diff > max
}
