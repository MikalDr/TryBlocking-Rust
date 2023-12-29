use bevy::prelude::*;
use self::systems::{spawn_player, player_movement};

pub mod component;
mod systems;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PLAYER_GROUNDED {
    is_grounded : bool
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PLAYER_GROUNDED  {is_grounded: true})
        .add_systems(Startup, (
            spawn_player,

        ))
        .add_systems(Update, (
            player_movement,
        ));
    }
}