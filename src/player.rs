use bevy::math::vec2;
use bevy::{math::vec3, prelude::*, utils::Instant};
use minigame::SetMinigameEvent;

use crate::terrain::{GroundTiles, TileComponent};
use crate::utils::*;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct WalkTrail(Instant);
#[derive(Component)]
struct Player;
#[derive(Resource)]
struct PlayerSpriteIndex(usize);
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Resource)]
struct TextureAtlasHandle(Option<Handle<TextureAtlas>>);
#[derive(Resource, Default)]
struct CurrentPlayerState(PlayerState);
#[derive(Resource)]
struct PlayerDirection(f32);
#[derive(Resource)]
struct WalkTrailTimer(Timer);
#[derive(Resource)]
struct DefaultAtlasHandle(pub Option<Handle<TextureAtlas>>);
#[derive(Resource, Default)]
pub struct CurrentPlayerChunkPos(pub (i32, i32));
#[derive(Event)]
pub struct PlayerChunkUpdateEvent(pub (i32, i32));

// TODO make this a state
#[derive(Default, PartialEq, Debug)]
enum PlayerState {
    #[default]
    Idle,
    Walk,
    Jump(Instant),
    Swim,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpriteIndex(0))
            .insert_resource(PlayerDirection(0.0))
            .insert_resource(CurrentPlayerState::default())
            .insert_resource(CurrentPlayerChunkPos::default())
            .insert_resource(WalkTrailTimer(Timer::from_seconds(
                WALK_TRAIL_TIMER,
                TimerMode::Repeating,
            )))
            .insert_resource(DefaultAtlasHandle(None))
            .add_event::<PlayerChunkUpdateEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_player_state)
            .add_systems(Update, camera_follow_player)
            .add_systems(Update, handle_player_hit_terrain)
            .add_systems(Update, handle_player_input)
            .add_systems(Update, spawn_walk_trail)
            .add_systems(Update, update_player_chunk_pos)
            .add_systems(Update, clean_old_walk_trails)
            .add_systems(Update, update_player_sprite);
    }
}

fn setup(
    mut commands: Commands,
    mut handle: ResMut<DefaultAtlasHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        Some(Vec2::splat(SPRITE_PADDING)),
        Some(Vec2::splat(SPRITE_SHEET_OFFSET)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    handle.0 = Some(texture_atlas_handle);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: handle.0.clone().unwrap(),
            sprite: TextureAtlasSprite::new(PLAYER_SPRITE_INDEX),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                .with_translation(vec3(0.0, 0.0, 2.0)),
            ..default()
        },
        Player,
        AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_INTERVAL,
            TimerMode::Repeating,
        )),
    ));
}

fn update_player_state(
    mut player_state: ResMut<CurrentPlayerState>,
    mut sprite_index: ResMut<PlayerSpriteIndex>,
    ground_tiles: Res<GroundTiles>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let transform = player_query.single_mut();
    let (x, y) = (transform.translation.x, transform.translation.y);
    let (x, y) = world_to_grid(x, y);
    let (x, y) = center_to_top_left_grid(x, y);
    let is_ground = ground_tiles.0.contains(&(x as i32, y as i32));

    if !is_ground && player_state.is_land() {
        player_state.0 = PlayerState::Jump(Instant::now());
    }
    if is_ground && player_state.0 == PlayerState::Swim {
        player_state.0 = PlayerState::Jump(Instant::now());
    }

    match player_state.0 {
        PlayerState::Jump(jumped_at) => {
            if jumped_at.elapsed().as_secs_f32() > PLAYER_JUMP_TIME {
                player_state.0 = if is_ground {
                    PlayerState::Idle
                } else {
                    PlayerState::Swim
                };
                sprite_index.0 = 0;
            }
        }
        _ => {}
    }
}

fn update_player_sprite(
    time: Res<Time>,
    mut sprite_index: ResMut<PlayerSpriteIndex>,
    player_state: Res<CurrentPlayerState>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, mut timer) = query.single_mut();
    timer.tick(time.delta());

    if player_state.is_walk() && timer.finished() {
        sprite_index.0 = (sprite_index.0 + 1) % 3;
    }
    if player_state.is_jump() && timer.finished() {
        sprite_index.0 = (sprite_index.0 + 1) % 3;
    }

    sprite.index = if player_state.is_land() {
        sprite_index.0 + PLAYER_SPRITE_INDEX
    } else if player_state.is_jump() {
        sprite_index.0 + PLAYER_SPRITE_INDEX + 3
    } else {
        49
    };
}

fn update_player_chunk_pos(
    mut chunk_pos: ResMut<CurrentPlayerChunkPos>,
    mut ev_chunk_update: EventWriter<PlayerChunkUpdateEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let transform = player_query.single();
    let (x, y) = (transform.translation.x, transform.translation.y);
    let (a, b) = world_to_grid(x, y);
    let (a, b) = center_to_top_left_grid(a, b);
    let (x, y) = grid_to_chunk(a, b);

    let (old_x, old_y) = chunk_pos.0;
    if old_x == x && old_y == y {
        return;
    }

    ev_chunk_update.send(PlayerChunkUpdateEvent((x, y)));
    chunk_pos.0 = (x, y);
}

fn handle_player_input(
    mut player_state: ResMut<CurrentPlayerState>,
    mut player_direction: ResMut<PlayerDirection>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }
    if player_state.is_jump() {
        return;
    }

    let mut transform = player_query.single_mut();

    let w_key = keys.pressed(KeyCode::W);
    let a_key = keys.pressed(KeyCode::A);
    let s_key = keys.pressed(KeyCode::S);
    let d_key = keys.pressed(KeyCode::D);

    let up_arrow_key = keys.pressed(KeyCode::Up);
    let left_arrow_key = keys.pressed(KeyCode::Left);
    let down_arrow_key = keys.pressed(KeyCode::Down);
    let right_arrow_key = keys.pressed(KeyCode::Right);

    let up_pressed = w_key || up_arrow_key;
    let left_pressed = a_key || left_arrow_key;
    let down_pressed = s_key || down_arrow_key;
    let right_pressed = d_key || right_arrow_key;

    let speed_scale = if keys.pressed(KeyCode::ShiftLeft) {
        5.0
    } else {
        1.0
    };
    let mut direction = Vec3::ZERO;

    if up_pressed {
        direction.y += 1.0;
    }
    if left_pressed {
        direction.x -= 1.0;
    }
    if down_pressed {
        direction.y -= 1.0;
    }
    if right_pressed {
        direction.x += 1.0;
    }

    if up_pressed || left_pressed || down_pressed || right_pressed {
        let player_angle = direction.y.atan2(direction.x);
        let sprite_angle = if player_state.is_land() {
            0.0
        } else {
            player_angle
        };
        let speed = if player_state.is_land() {
            PLAYER_SPEED
        } else {
            PLAYER_FISH_SPEED
        };
        let new_pos = transform.translation + direction.normalize() * speed * speed_scale;

        if !new_pos.is_nan() {
            transform.translation = new_pos;
        }

        transform.rotation = Quat::from_rotation_z(sprite_angle);
        player_direction.0 = player_angle;
        player_state.0 = if player_state.is_land() {
            PlayerState::Walk
        } else {
            PlayerState::Swim
        };
    } else {
        player_state.0 = if player_state.is_land() {
            PlayerState::Idle
        } else {
            PlayerState::Swim
        };
    }
}

fn spawn_walk_trail(
    time: Res<Time>,
    mut commands: Commands,
    player_state: Res<CurrentPlayerState>,
    player_angle: Res<PlayerDirection>,
    image_handle: Res<DefaultAtlasHandle>,
    mut timer: ResMut<WalkTrailTimer>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());
    if player_query.is_empty() {
        return;
    }

    if !timer.0.finished() || !player_state.is_walk() {
        return;
    }

    let transform = player_query.single_mut();
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: image_handle.0.clone().unwrap(),
            sprite: TextureAtlasSprite::new(50),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32 - 1.0))
                .with_translation(vec3(transform.translation.x, transform.translation.y, 1.0))
                .with_rotation(Quat::from_rotation_z(player_angle.0)),
            ..default()
        },
        WalkTrail(Instant::now()),
    ));
}

fn clean_old_walk_trails(
    mut commands: Commands,
    query: Query<(Entity, &WalkTrail), With<WalkTrail>>,
) {
    if query.is_empty() {
        return;
    }

    for (entity, trail) in query.iter() {
        if trail.0.elapsed().as_secs_f32() > TRAIL_LIFE_SPAN {
            commands.entity(entity).despawn();
        }
    }
}

fn camera_follow_player(
    mut cam_query: Query<(&Camera, &mut Transform), Without<Player>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (_, mut cam_transform) = cam_query.get_single_mut().unwrap();
    let player_transform = player_query.get_single_mut().unwrap();

    cam_transform.translation = cam_transform.translation.lerp(
        vec3(
            player_transform.translation.x,
            player_transform.translation.y,
            0.0,
        ),
        0.05,
    );
    // cam_transform.translation = player_transform.translation;
}

fn handle_player_hit_terrain(
    player_query: Query<&Transform, With<Player>>,
    mut terrain_query: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<TileComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut set_minigame_event_writer: EventWriter<SetMinigameEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (terrain_entity, terrain_transform, terrain_ta_sprite) in terrain_query.iter_mut() {
            if !keyboard_input.just_pressed(KeyCode::Z) {
                continue;
            }

            let x_colliding = !diff_exceeds_max(
                player_transform.translation.x,
                terrain_transform.translation.x,
                TILE_W as f32 * 2.0,
            );
            let y_colliding = !diff_exceeds_max(
                player_transform.translation.y,
                terrain_transform.translation.y,
                TILE_H as f32 * 2.0,
            );
            if !x_colliding || !y_colliding {
                continue;
            }

            if terrain_ta_sprite.index == FOUR_WINDOWED_HOUSE_SPRITE_INDEX {
                set_minigame_event_writer.send(SetMinigameEvent {
                    minigame_state: minigame::MinigameState::Maze,
                    seed: terrain_entity.index(),
                });
            }
        }
    }
}

impl CurrentPlayerState {
    fn is_land(&self) -> bool {
        match self.0 {
            PlayerState::Idle => true,
            PlayerState::Walk => true,
            _ => false,
        }
    }

    fn is_walk(&self) -> bool {
        self.0 == PlayerState::Walk
    }

    fn is_jump(&self) -> bool {
        match self.0 {
            PlayerState::Jump(_) => true,
            _ => false,
        }
    }
}
