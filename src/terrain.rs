use std::time::Duration;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::{HashMap, HashSet};
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::player::{CurrentPlayerChunkPos, PlayerChunkUpdateEvent};
use crate::utils::*;
use crate::*;

#[derive(Component)]
struct TileComponent;
#[derive(Resource)]
pub struct GroundTiles(pub HashSet<(i32, i32)>);
#[derive(Resource)]
struct CurrentChunks(HashMap<(i32, i32), Vec<Entity>>);
#[derive(Resource)]
struct GenerationSeed(u32);
#[derive(Event)]
pub struct ResetTerrainEvent;

#[derive(Eq, PartialEq, Hash)]
struct Tile {
    pos: (i32, i32),
    sprite: usize,
    z_index: i32,
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        app.insert_resource(GroundTiles(HashSet::new()))
            .insert_resource(CurrentChunks(HashMap::new()))
            .insert_resource(GenerationSeed(rng.gen()))
            .add_systems(Update, handle_terrain_reset_event)
            .add_systems(Update, despawn_chunks)
            .add_systems(
                Update,
                clean_ground_tiles.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
            .add_systems(Update, handle_player_chunk_update_event)
            .add_event::<ResetTerrainEvent>();
    }
}

fn handle_terrain_reset_event(
    mut commands: Commands,
    mut reader: EventReader<ResetTerrainEvent>,
    mut ev_writer: EventWriter<PlayerChunkUpdateEvent>,
    player_pos: Res<CurrentPlayerChunkPos>,
    mut chunks: ResMut<CurrentChunks>,
    mut ground_tiles: ResMut<GroundTiles>,
    mut seed: ResMut<GenerationSeed>,
    tile_q: Query<Entity, With<TileComponent>>,
) {
    if reader.is_empty() {
        return;
    }

    reader.clear();
    for t in tile_q.iter() {
        commands.entity(t).despawn();
    }

    // Reset res
    chunks.0.clear();
    ground_tiles.0.clear();

    let mut rng = rand::thread_rng();
    seed.0 = rng.gen();

    // Trigger world re-generation
    let (x, y) = player_pos.0;
    ev_writer.send(PlayerChunkUpdateEvent((x, y)));
}

fn clean_ground_tiles(
    player_pos: Res<CurrentPlayerChunkPos>,
    mut ground_tiles: ResMut<GroundTiles>,
) {
    let (x, y) = player_pos.0;
    ground_tiles.0.retain(|pos| {
        let (px, py) = grid_to_chunk(pos.0 as f32, pos.1 as f32);
        px.abs_diff(x) <= 1 || py.abs_diff(y) <= 1
    });
}

fn despawn_chunks(
    mut commands: Commands,
    mut current_chunks: ResMut<CurrentChunks>,
    player_pos: Res<CurrentPlayerChunkPos>,
) {
    let mut keys_to_remove = Vec::new();
    let (x, y) = player_pos.0;

    for ((cx, cy), entities) in current_chunks.0.iter() {
        if cx.abs_diff(x) <= 1 && cy.abs_diff(y) <= 1 {
            continue;
        }

        for e in entities.iter() {
            commands.entity(*e).despawn();
        }
        keys_to_remove.push((*cx, *cy));
    }

    for (cx, cy) in keys_to_remove {
        current_chunks.0.remove(&(cx, cy));
    }
}

fn handle_player_chunk_update_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seed: Res<GenerationSeed>,
    mut current_chunks: ResMut<CurrentChunks>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_chunk_update: EventReader<PlayerChunkUpdateEvent>,
    mut ground_tiles: ResMut<GroundTiles>,
) {
    if ev_chunk_update.is_empty() {
        return;
    }

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

    for new_chunk_pos in ev_chunk_update.read() {
        let (x, y) = new_chunk_pos.0;

        let chunk_nei = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, 1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (0, 0),
        ];
        let mut tiles = HashSet::new();
        let mut ground_map = HashSet::new();

        for (i, j) in chunk_nei.iter() {
            let (x, y) = (x + *i, y + *j);
            if current_chunks.0.contains_key(&(x, y)) {
                continue;
            }

            let start = (x * CHUNK_W as i32, y * CHUNK_H as i32);
            let (chunk_tiles, chunk_ground_map) = gen_chunk(seed.0, (start.0, start.1));
            tiles.extend(chunk_tiles);
            ground_map.extend(chunk_ground_map);
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
            tiles.insert(Tile::new((*x, *y), tile, 0));
        }
        ground_tiles.0.extend(updated_ground_map);

        for t in tiles.iter() {
            let (cx, cy) = grid_to_chunk(t.pos.0 as f32, t.pos.1 as f32);
            let (x, y) = grid_to_world(t.pos.0 as f32, t.pos.1 as f32);
            let (x, y) = center_to_top_left(x, y);

            let e = commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: handle.clone(),
                        sprite: TextureAtlasSprite::new(t.sprite),
                        transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                            .with_translation(vec3(x, y, t.z_index as f32)),
                        ..default()
                    },
                    TileComponent,
                ))
                .id();

            current_chunks
                .0
                .entry((cx, cy))
                .or_insert_with(Vec::new)
                .push(e);
        }
    }
}

fn gen_chunk(gen_seed: u32, start: (i32, i32)) -> (HashSet<Tile>, HashSet<(i32, i32)>) {
    let mut rng = rand::thread_rng();
    let noise = Perlin::new(gen_seed);

    let mut tiles = HashSet::new();
    let mut ground_map = HashSet::new();
    let end = (start.0 + CHUNK_W as i32, start.1 + CHUNK_H as i32);
    for x in start.0 - 1..end.0 + 1 {
        for y in start.1 - 1..end.1 + 1 {
            let noise_val1 = noise.get([x as f64 / 100.5, y as f64 / 100.5]);
            let noise_val2 = noise.get([x as f64 / 53.5, y as f64 / 53.5]);
            let noise_val3 = noise.get([x as f64 / 43.5, y as f64 / 43.5]);
            let noise_val4 = noise.get([x as f64 / 23.5, y as f64 / 23.5]);
            let noise_val = (noise_val1 + noise_val2 + noise_val3 + noise_val4) / 4.0;
            let chance = rng.gen_range(0.0..1.0);

            // Ground
            if noise_val > 0.0 {
                ground_map.insert((x, y));
            } else {
                continue;
            }

            // Too close to shore
            if noise_val < 0.05 {
                continue;
            }

            // Dense Forest
            if (noise_val > 0.5 || noise_val3 > 0.98) && chance > 0.2 {
                tiles.insert(Tile::new((x, y), 27, 5));
                continue;
            }
            // Patch Forest
            if noise_val3 > 0.5 && noise_val < 0.5 && chance > 0.4 {
                let chance2 = rng.gen_range(0.0..1.0);
                let tile = if chance2 > 0.7 {
                    rng.gen_range(24..=26)
                } else {
                    rng.gen_range(24..=25)
                };
                tiles.insert(Tile::new((x, y), tile, 3));
                continue;
            }
            // Sparse Forest
            if noise_val4 > 0.4 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.9 {
                let chance = rng.gen_range(0.0..1.0);
                let tile = if chance > 0.78 {
                    rng.gen_range(28..=29)
                } else {
                    rng.gen_range(24..=25)
                };
                tiles.insert(Tile::new((x, y), tile, 3));
                continue;
            }

            // Bones
            if noise_val > 0.3 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.98 {
                let tile = rng.gen_range(40..=43);
                tiles.insert(Tile::new((x, y), tile, 1));
                continue;
            }

            // Settlements
            if noise_val > 0.1 && noise_val < 0.3 && noise_val3 < 0.4 && chance > 0.8 {
                let chance2 = rng.gen_range(0.0..1.0);

                if chance2 > 0.98 {
                    let chance3 = rng.gen_range(0.0..1.0);
                    let tile = if chance3 > 0.75 {
                        rng.gen_range(18..=19)
                    } else {
                        rng.gen_range(16..=17)
                    };
                    tiles.insert(Tile::new((x, y), tile, 8));
                } else {
                    if noise_val > 0.2 && noise_val < 0.3 && noise_val3 < 0.3 && chance > 0.9 {
                        tiles.insert(Tile::new((x, y), 32, 1));
                    }
                }

                continue;
            }

            // Color Check
            // if noise_val > 0.1 && noise_val4 < 0.5 {
            //     tiles.insert(Tile::new((x, y), 64, 1));
            //     continue;
            // }
        }
    }

    (tiles, ground_map)
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
        [0, 1, 1, 0] => 3,
        [1, 0, 1, 0] => 4,
        [0, 1, 0, 1] => 1,
        [1, 0, 0, 1] => 2,
        _ => 0,
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
