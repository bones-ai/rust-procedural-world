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
