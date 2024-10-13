use std::{env, fs, io};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{close_on_esc, WindowMode},
};
use bevy_pancam::{PanCam, PanCamPlugin};

use island_procgen::{minigame::MinigamePlugin, player::PlayerPlugin, terrain::TerrainPlugin};
use island_procgen::{terrain::ResetTerrainEvent, *};
use terrain::GenerationSeed;
use utils::seed_from_seed_str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut mode = WindowMode::default();
    if !args.contains(&ARG_DISABLE_FULLSCREEN.to_string()) {
        mode = bevy::window::WindowMode::Fullscreen;
    }

    let seed = init_seed();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode,
                        resolution: (WW as f32, WH as f32).into(),
                        title: "ProcGen".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(GenerationSeed(seed))
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        )))
        .add_plugins(PanCamPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(TerrainPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MinigamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, handle_settings_input)
        .add_systems(Update, close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

fn handle_settings_input(keys: Res<Input<KeyCode>>, mut writer: EventWriter<ResetTerrainEvent>) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    writer.send(ResetTerrainEvent);
}

fn init_seed() -> u32 {
    let seed_str = if fs::exists(SEED_FILE_PATH).unwrap() {
        let content =
            String::from_utf8(fs::read(SEED_FILE_PATH).expect("Failed to read seed file"))
                .expect("Failed to encode seed file");

        if content.is_empty() {
            prompt_seed_str_input()
        } else {
            content
        }
    } else {
        let input = prompt_seed_str_input();
        fs::write(SEED_FILE_PATH, &input).expect("Failed to write seed file");

        input
    };

    seed_from_seed_str(seed_str)
}

fn prompt_seed_str_input() -> String {
    let mut input = String::new();
    println!("Please enter world seed, and then press ENTER:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed read to input");

    input.trim().to_owned()
}
