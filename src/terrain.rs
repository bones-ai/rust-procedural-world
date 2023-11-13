use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::utils::HashSet;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::utils::*;
use crate::*;

#[derive(Component)]
struct TileComponent;
#[derive(Resource)]
pub struct GroundTiles(pub HashSet<(i32, i32)>);
#[derive(Event)]
pub struct ResetTerrainEvent;

struct Tile {
    pos: (i32, i32),
    sprite: usize,
    z_index: i32,
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, handle_terrain_reset_event)
            .insert_resource(GroundTiles(HashSet::new()))
            .add_event::<ResetTerrainEvent>();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ground_tiles: ResMut<GroundTiles>,
) {
    gen_random_world(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut ground_tiles,
    );
}

fn handle_terrain_reset_event(
    mut commands: Commands,
    mut reader: EventReader<ResetTerrainEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ground_tiles: ResMut<GroundTiles>,
    tile_q: Query<Entity, With<TileComponent>>,
) {
    if reader.is_empty() {
        return;
    }

    reader.clear();
    for t in tile_q.iter() {
        commands.entity(t).despawn();
    }
    gen_random_world(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut ground_tiles,
    );
}

fn gen_random_world(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ground_tiles: &mut ResMut<GroundTiles>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen());
    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        Some(Vec2::splat(SPRITE_PADDING)),
        Some(Vec2::splat(SPRITE_SHEET_OFFSET)),
    );
    let handle = texture_atlases.add(texture_atlas);

    let mut tiles = Vec::new();
    let mut ground_map = HashSet::new();
    for x in 0..GRID_COLS as i32 {
        for y in 0..GRID_ROWS as i32 {
            let noise_val = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
            let chance = rng.gen_range(0.0..1.0);

            // Ground
            if noise_val > 0.1 {
                ground_map.insert((x, y));
            }

            // Mountains
            if noise_val > 0.3 && noise_val < 0.31 {
                tiles.push(Tile::new((x, y), 4, 6));
            }

            // Trees
            if noise_val > 0.32 && noise_val < 0.6 {
                if chance > 0.9 {
                    tiles.push(Tile::new((x, y), rng.gen_range(1..=3), 5));
                } else if chance > 0.8 {
                    tiles.push(Tile::new((x, y), 0, 5));
                }
            }

            // Bones
            if noise_val > 0.6 && noise_val < 0.7 && chance > 0.98 {
                let bone_tile = if rng.gen_range(0.0..1.0) > 0.5 { 6 } else { 7 };
                tiles.push(Tile::new((x, y), bone_tile, 2));
            }

            // Houses
            if noise_val > 0.7 && chance > 0.97 {
                let house_tile = if rng.gen_range(0.0..1.0) > 0.85 { 13 } else { 12 };
                tiles.push(Tile::new((x, y), house_tile, 4));
            }
        }
    }

    let mut updated_ground_map = HashSet::new();
    for (x, y) in ground_map.iter() {
        let (num_nei, tile) = process_tile((*x, *y), &ground_map);
        if num_nei == 1 {
            continue;
        }

        // Ignore edges
        // This will help in better player visualization when going from land to water
        updated_ground_map.insert((*x, *y));
        tiles.push(Tile::new((*x, *y), tile, 0));
    }
    ground_map = updated_ground_map;

    for t in tiles.iter() {
        let (x, y) = grid_to_world(t.pos.0 as f32, t.pos.1 as f32);
        let (x, y) = center_to_top_left(x, y);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: handle.clone(),
                sprite: TextureAtlasSprite::new(t.sprite),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                    .with_translation(vec3(x, y, t.z_index as f32)),
                ..default()
            },
            TileComponent,
        ));
    }

    ground_tiles.0.clear();
    ground_tiles.0 = ground_map;
}

fn process_tile((x, y): (i32, i32), occupied: &HashSet<(i32, i32)>) -> (i32, usize) {
    let nei_options = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nei = [1, 1, 1, 1];
    let mut nei_count = 4;
    for (idx, (i, j)) in nei_options.iter().enumerate() {
        if !occupied.contains(&(x + i, y + j)) {
            nei[idx] = 0;
            nei_count -= 1;
        }
    }

    let tile = match nei {
        [1, 0, 0, 1] => 25,
        [0, 1, 0, 1] => 24,
        [0, 1, 1, 0] => 26,
        [1, 0, 1, 0] => 27,
        _ => 30,
    };

    (nei_count, tile)
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
