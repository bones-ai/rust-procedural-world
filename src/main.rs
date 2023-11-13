use bevy::{math::vec3, prelude::*, utils::HashSet, window::close_on_esc};
use bevy_pancam::{PanCam, PanCamPlugin};
use noise::{NoiseFn, Perlin};
use rand::Rng;

// Sprite
const SPRITE_SHEET_PATH: &str = "sprite-sheet.png";
const TILE_W: usize = 6;
const TILE_H: usize = 8;
const SPRITE_SHEET_W: usize = 36 / TILE_W;
const SPRITE_SHEET_H: usize = 40 / TILE_H;
const SPRITE_SCALE_FACTOR: usize = 2;

// Window
const GRID_COLS: usize = 200;
const GRID_ROWS: usize = 100;
const GEN_W: usize = GRID_COLS * TILE_W;
const GEN_H: usize = GRID_ROWS * TILE_H;
const BG_COLOR: (u8, u8, u8) = (194, 195, 199);

// Perlin
const NOISE_SCALE: f64 = 12.5;

#[derive(Component)]
struct TileComponent;

struct Tile {
    pos: (i32, i32),
    sprite: usize,
    z_index: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PanCamPlugin)
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 255,
        )))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, close_on_esc)
        .run();
}

fn handle_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    tiles_query: Query<Entity, With<TileComponent>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    for entity in tiles_query.iter() {
        commands.entity(entity).despawn();
    }
    gen_world(&mut commands, asset_server, &mut texture_atlases);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(GEN_W as f32, GEN_H as f32, 0.0),
            ..Default::default()
        })
        .insert(PanCam::default());

    gen_world(&mut commands, asset_server, &mut texture_atlases);
}

fn gen_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen());

    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut tiles = Vec::new();
    let mut occupied = HashSet::new();
    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            let noise_val = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
            let choice = rng.gen_range(0.0..1.0);
            let (x, y) = (x as i32, y as i32);

            // Ground
            if noise_val > 0.1 {
                occupied.insert((x, y));
            }

            // Mountains
            if noise_val > 0.3 && noise_val < 0.31 {
                tiles.push(Tile::new((x, y), 8, 1));
            }

            // Trees
            if noise_val > 0.35 && noise_val < 0.6 {
                if choice > 0.9 {
                    tiles.push(Tile::new((x, y), rng.gen_range(7..=9), 1));
                } else if choice > 0.8 {
                    tiles.push(Tile::new((x, y), 6, 1));
                }
            }

            // Bones
            if noise_val > 0.6 && noise_val < 0.7 && choice > 0.98 {
                tiles.push(Tile::new((x, y), rng.gen_range(18..=19), 1));
            }

            // Houses
            if noise_val > 0.7 && choice > 0.97 {
                let house_tile = if rng.gen_range(0.0..1.0) < 0.85 { 12 } else { 13 };
                tiles.push(Tile::new((x, y), house_tile, 1));
            }
        }
    }

    for (x, y) in occupied.iter() {
        let (tile, nei_count) = get_tile((*x, *y), &occupied);
        if nei_count == 1 {
            continue;
        }

        tiles.push(Tile::new((*x, *y), tile, 0));
    }

    for tile in tiles.iter() {
        let (x, y) = tile.pos;
        let (x, y) = grid_to_world(x as f32, y as f32);
        // let (x, y) = center_to_top_left(x as f32, y as f32);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(tile.sprite),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                    .with_translation(vec3(x, y, tile.z_index as f32)),
                ..default()
            },
            TileComponent,
        ));
    }
}

fn get_tile((x, y): (i32, i32), occupied: &HashSet<(i32, i32)>) -> (usize, i32) {
    let (x, y) = (x as i32, y as i32);
    let nei_options = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nei = [1, 1, 1, 1];
    let mut nei_count = 0;

    for (idx, (i, j)) in nei_options.iter().enumerate() {
        if occupied.contains(&(x + i, y + j)) {
            nei_count += 1;
            continue;
        }

        nei[idx] = 0;
    }

    let tile = match nei {
        [0, 1, 1, 0] => 1,
        [1, 0, 1, 0] => 2,
        [0, 1, 0, 1] => 3,
        [1, 0, 0, 1] => 4,
        _ => 0,
    };

    (tile, nei_count)
}

fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_W as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_H as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}

fn center_to_top_left(x: f32, y: f32) -> (f32, f32) {
    let x_center = x - (GRID_COLS as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0;
    let y_center = ((GRID_COLS as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0) - y;

    (x_center, y_center)
}

impl Tile {
    fn new(pos: (i32, i32), sprite: usize, z_index: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
        }
    }
}
