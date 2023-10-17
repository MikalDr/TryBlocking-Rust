use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use platform::PlatformPlugin;
use player::PlayerPlugin;

mod player;
mod platform;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest(),), 
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
    ))
    .add_systems(Startup, spawn_camera)
    .add_plugins((
        PlayerPlugin, 
        PlatformPlugin
    ))
    .run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width()/2.0 , window.height()/2.0, 0.0),
        ..default()
    });
}