use bevy::{prelude::*};
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use platform::PlatformPlugin;
use player::PlayerPlugin;
use camera::CameraPlugin;

mod player;
mod platform;
mod camera;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest(),), 
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
    ))
    .add_plugins((
        CameraPlugin,
        PlayerPlugin, 
        PlatformPlugin,
    ))
    .run();
}