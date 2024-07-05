use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowMode;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelSelection, LevelSpawnBehavior};
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "repro".into(),
                name: Some("repro".into()),
                resizable: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(Light2dPlugin)
        .add_systems(Startup, spawn_camera)
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, spawn_ldtk_world)
        .insert_resource(LevelSelection::index(0))
        .run();
}
fn spawn_ldtk_world(mut command: Commands, asset_server: Res<AssetServer>, mut level_plugin: ResMut<LdtkSettings>) {
    // level_plugin.level_spawn_behavior = LevelSpawnBehavior::UseZeroTranslation;
    level_plugin.level_spawn_behavior = LevelSpawnBehavior::UseWorldTranslation { load_level_neighbors: true };
    command.spawn((LdtkWorldBundle {
        ldtk_handle: asset_server.load("tbg.ldtk"),
        ..Default::default()
    }, Name::new("World")));
}

#[derive(Component, Reflect, Debug, Default)]
pub struct PrimaryCamera;
fn spawn_camera(mut command: Commands) {
    command.spawn((Camera2dBundle::default(), AmbientLight2d {
        brightness: 0.1,
        ..default()
    }, PrimaryCamera::default()));
}
