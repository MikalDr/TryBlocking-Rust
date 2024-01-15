use bevy::prelude::*;
use self::systems::{spawn_player, player_movement,player_ground_check};

pub mod component;
mod systems;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PLAYER_RESOURCES {
    pub air_timer: f32,
    pub is_ground: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PLAYER_RESOURCES  {air_timer: 0.0, is_ground: false})
        .add_systems(Startup, (
            spawn_player,

        ))
        .add_systems(Update, (
            player_movement,
            player_ground_check,
        ));
    }
}