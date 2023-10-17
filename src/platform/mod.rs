use bevy::prelude::*;

use self::systems::{spawn_platform, display_events};

mod systems;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (
            spawn_platform,
        ))
        .add_systems(Update, (
            display_events,
        ))
        /* .add_systems(Update, (
            
        ))*/
        ;
    }
}
