use std::vec;

use bevy::prelude::*;

use crate::{configs::*, maze::*, utils::*};

const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MinigameState {
    #[default]
    None,
    Maze,
}

#[derive(Component)]
struct Minigame;
#[derive(Component)]
struct CloseMinigameButton;
#[derive(Component)]
struct MinigamePlayer;
#[derive(Component)]
struct MinigameContainer;

#[derive(Event)]
pub struct SetMinigameEvent {
    pub minigame_state: MinigameState,
    pub seed: u32,
}

pub struct MinigamePlugin;

impl Plugin for MinigamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MinigameState>()
            .add_event::<SetMinigameEvent>()
            .add_systems(Update, handle_set_minigame)
            .add_systems(Update, handle_minigame_player_input)
            .add_systems(Update, interact_with_close_minigame_button)
            .add_systems(OnEnter(MinigameState::None), despawn_minigame);
    }
}

impl Minigame {
    fn spawn_new(commands: &mut Commands, minigame_state: &MinigameState, seed: u32) {
        match minigame_state {
            MinigameState::Maze => Self::spawn_new_maze(commands, seed),
            _ => (),
        }
    }

    fn spawn_new_maze(commands: &mut Commands, seed: u32) {
        // TODO: create a maze UI for the player to navigate through
        let maze = Maze::new(20, 12, seed);

        // Minigame
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
                Minigame,
            ))
            .with_children(|parent| {
                // Maze
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::GREEN.into(),
                            ..default()
                        },
                        maze.clone(),
                    ))
                    .with_children(|parent| {
                        // Rows
                        for row in maze.cells {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::BLUE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Cells
                                    for cell in row {
                                        let mut border = UiRect { ..default() };

                                        if cell.top_wall {
                                            border.top = Val::Px(2.0);
                                        }
                                        if cell.bottom_wall {
                                            border.bottom = Val::Px(2.0);
                                        }
                                        if cell.left_wall {
                                            border.left = Val::Px(2.0);
                                        }
                                        if cell.right_wall {
                                            border.right = Val::Px(2.0);
                                        }

                                        parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    height: Val::Px(50.0),
                                                    width: Val::Px(50.0),
                                                    border,
                                                    ..default()
                                                },
                                                background_color: Color::ORANGE.into(),
                                                border_color: Color::BLACK.into(),
                                                ..default()
                                            },
                                            cell,
                                        ));
                                    }
                                });
                        }

                        // Minigame Player Container
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        height: Val::Percent(100.0),
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                MinigameContainer,
                            ))
                            .with_children(|parent| {
                                // Minigame Player
                                parent.spawn((
                                    // SpriteSheetBundle {
                                    //     // texture_atlas: handle.0.clone().unwrap(),
                                    //     sprite: TextureAtlasSprite::new(PLAYER_SPRITE_INDEX),
                                    //     transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                                    //         .with_translation(vec3(0.0, 0.0, 4.0)),
                                    //     ..default()
                                    // },
                                    NodeBundle {
                                        style: Style {
                                            height: Val::Px(MAZE_PLAYER_HEIGHT),
                                            width: Val::Px(MAZE_PLAYER_WIDTH),
                                            top: Val::Px(0.0),
                                            left: Val::Px(0.0),
                                            ..default()
                                        },
                                        background_color: Color::RED.into(),
                                        ..default()
                                    },
                                    MinigamePlayer,
                                    // AnimationTimer(Timer::from_seconds(
                                    //     PLAYER_ANIMATION_INTERVAL,
                                    //     TimerMode::Repeating,
                                    // )),
                                ));
                            });
                    });

                // Close Button
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                height: Val::Px(80.0),
                                width: Val::Px(200.0),
                                padding: UiRect {
                                    left: Val::Px(10.0),
                                    right: Val::Px(10.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                margin: UiRect {
                                    left: Val::Px(10.0),
                                    right: Val::Px(10.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..default()
                            },
                            background_color: NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        CloseMinigameButton {},
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Close",
                                    TextStyle {
                                        font_size: 32.0,
                                        color: Color::BLACK,
                                        ..default()
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
    }
}

fn handle_set_minigame(
    mut commands: Commands,
    mut set_minigame_event_reader: EventReader<SetMinigameEvent>,
) {
    for event in set_minigame_event_reader.read() {
        commands.insert_resource(NextState(Some(event.minigame_state)));
        Minigame::spawn_new(&mut commands, &event.minigame_state, event.seed);
        return;
    }
}

fn despawn_minigame(mut commands: Commands, minigame_query: Query<Entity, With<Minigame>>) {
    if let Ok(minigame_entity) = minigame_query.get_single() {
        commands.entity(minigame_entity).despawn_recursive();
    }
}

fn handle_minigame_player_input(
    mut minigame_player_query: Query<&mut Style, With<MinigamePlayer>>,
    minigame_container_query: Query<&mut Node, With<MinigameContainer>>,
    maze_query: Query<&Maze>,
    keys: Res<Input<KeyCode>>,
) {
    if minigame_player_query.is_empty()
        || minigame_container_query.is_empty()
        || maze_query.is_empty()
    {
        return;
    }

    let w_key = keys.pressed(KeyCode::W);
    let s_key = keys.pressed(KeyCode::S);
    let a_key = keys.pressed(KeyCode::A);
    let d_key = keys.pressed(KeyCode::D);

    let up_arrow_key = keys.pressed(KeyCode::Up);
    let down_arrow_key = keys.pressed(KeyCode::Down);
    let left_arrow_key = keys.pressed(KeyCode::Left);
    let right_arrow_key = keys.pressed(KeyCode::Right);

    let up_pressed = w_key || up_arrow_key;
    let down_pressed = s_key || down_arrow_key;
    let left_pressed = a_key || left_arrow_key;
    let right_pressed = d_key || right_arrow_key;

    if !(up_pressed || down_pressed || left_pressed || right_pressed) {
        return;
    }

    let mut style = minigame_player_query.get_single_mut().unwrap();
    let container = minigame_container_query.get_single().unwrap();
    let maze = maze_query.get_single().unwrap();

    let speed_scale = if keys.pressed(KeyCode::ShiftLeft) {
        5.0
    } else {
        1.0
    };

    let px = MAZE_PLAYER_SPEED * speed_scale;

    let mut new_left = style.left.clone();
    let mut new_top = style.top.clone();

    if up_pressed {
        new_top = add_px_vals(style.top, Val::Px(-px));
        if !px_val_between(
            new_top,
            Val::Px(0.0),
            Val::Px(container.size().y - MAZE_PLAYER_HEIGHT),
        ) {
            return;
        }
    }
    if down_pressed {
        new_top = add_px_vals(style.top, Val::Px(px));
        if !px_val_between(
            new_top,
            Val::Px(0.0),
            Val::Px(container.size().y - MAZE_PLAYER_HEIGHT),
        ) {
            return;
        }
    }
    if left_pressed {
        new_left = add_px_vals(style.left, Val::Px(-px));
        if !px_val_between(
            new_left,
            Val::Px(0.0),
            Val::Px(container.size().x - MAZE_PLAYER_WIDTH),
        ) {
            return;
        }
    }
    if right_pressed {
        new_left = add_px_vals(style.left, Val::Px(px));
        if !px_val_between(
            new_left,
            Val::Px(0.0),
            Val::Px(container.size().x - MAZE_PLAYER_WIDTH),
        ) {
            return;
        }
    }

    for (curr_x, curr_y) in
        // Find the current (x,y) of each corner of the minigame player
        minigame_player_pos_all_vertices(style.left, style.top, style.width, style.height)
    {
        for (new_x, new_y) in
            // Find the next (x,y) the corner is trying to go to
            minigame_player_pos_all_vertices(new_left, new_top, style.width, style.height)
        {
            // Check if wall between the two cells
            if maze.is_wall_between((curr_x, curr_y), (new_x, new_y)) {
                return;
            }
        }
    }

    style.left = new_left;
    style.top = new_top;
}

fn get_minigame_player_pos(minigame_player_left: Val, minigame_player_top: Val) -> (usize, usize) {
    let mut x: usize = 0;
    loop {
        let cell_left = Val::Px(MAZE_CELL_WIDTH * (x as f32 + 1.0));
        if px_val_greater_than(cell_left, minigame_player_left) {
            break;
        }
        x += 1;
    }

    let mut y: usize = 0;
    loop {
        let cell_top = Val::Px(MAZE_CELL_HEIGHT * (y as f32 + 1.0));
        if px_val_greater_than(cell_top, minigame_player_top) {
            break;
        }
        y += 1;
    }

    (x, y)
}

fn minigame_player_pos_all_vertices(
    left: Val,
    top: Val,
    width: Val,
    height: Val,
) -> Vec<(usize, usize)> {
    vec![
        get_minigame_player_pos(left, top),
        get_minigame_player_pos(add_px_vals(left, width), top),
        get_minigame_player_pos(left, add_px_vals(top, height)),
        get_minigame_player_pos(add_px_vals(left, width), add_px_vals(top, height)),
    ]
}

fn interact_with_close_minigame_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CloseMinigameButton>),
    >,
    mut minigame_state_next_state: ResMut<NextState<MinigameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                minigame_state_next_state.set(MinigameState::None);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
