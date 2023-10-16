use bevy::prelude::*;

use self::systems::spawn_player;

mod component;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player);
    }
}