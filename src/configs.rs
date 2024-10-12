// Sprites
pub const TILE_W: usize = 6;
pub const TILE_H: usize = 8;
pub const SPRITE_SHEET_W: usize = 8;
pub const SPRITE_SHEET_H: usize = 9;
pub const SPRITE_SCALE_FACTOR: usize = 5;
pub const ONE_WINDOWED_HOUSE_SPRITE_INDEX: usize = 17;
pub const FOUR_WINDOWED_HOUSE_SPRITE_INDEX: usize = 18;
pub const PLAYER_SPRITE_INDEX: usize = 56;
pub const SPRITE_SHEET_PATH: &str = "sprite-sheet.png";
pub const SPRITE_PADDING: f32 = 2.0;
pub const SPRITE_SHEET_OFFSET: f32 = 2.0;

// Window
pub const GRID_COLS: usize = 1000;
pub const GRID_ROWS: usize = 800;
pub const GRID_W: usize = GRID_COLS * TILE_W;
pub const GRID_H: usize = GRID_ROWS * TILE_H;
pub const WW: usize = 1920;
pub const WH: usize = 1080;
pub const BG_COLOR: (u8, u8, u8) = (181, 212, 220);

// Chunk
pub const CHUNK_W: usize = 120;
pub const CHUNK_H: usize = 100;

// Player
pub const PLAYER_SPEED: f32 = 1.0;
pub const PLAYER_FISH_SPEED: f32 = 1.5;
pub const PLAYER_ANIMATION_INTERVAL: f32 = 0.3;
pub const WALK_TRAIL_TIMER: f32 = 1.2;
pub const TRAIL_LIFE_SPAN: f32 = 5.0;
pub const PLAYER_JUMP_TIME: f32 = 0.3;

// Minigame
pub const CELL_HEIGHT: f32 = 50.0;
pub const CELL_WIDTH: f32 = 50.0;
pub const MINIGAME_PLAYER_HEIGHT: f32 = 20.0;
pub const MINIGAME_PLAYER_WIDTH: f32 = 20.0;
pub const MINIGAME_PLAYER_SPEED: f32 = 2.0;

// Args
pub const ARG_DISABLE_FULLSCREEN: &str = "no-fullscreen";
