use bevy::prelude::*;

use crate::maze::Maze;

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
        let maze = Maze::new(12, 8, seed);

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
