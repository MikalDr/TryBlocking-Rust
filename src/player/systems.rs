use bevy::{prelude::*, window::PrimaryWindow, input::keyboard};
use bevy_rapier2d::prelude::*;

use super::component::Player;

const SPRITE_PATH: &str = "TemPlayer.png";
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
#[derive(Resource)]
pub struct PLAYER_GROUNDED {
    is_grounded : bool
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0 , window.height()/2.0, 0.0),
                texture: asset_server.load(SPRITE_PATH),
                ..default()
            },
            Player {}, 
            RigidBody::Dynamic,
            Collider::ball(10.0),
        ),    
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    is_player_grounded: Res<PLAYER_GROUNDED>,
)
{
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if keyboard_input.just_pressed(KeyCode::Space) &&  is_player_grounded.is_grounded{
            
        }
        /*
        == NEEDED IF TOPDOWN GAME ==
        if keyboard_input.just_pressed(KeyCode::Up) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        */

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

/*
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
){
    let window = window_query.get_single().unwrap();

    if let Ok(player_transform) = player_query.get_single_mut() {
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;
    
        let mut translation = player_transform.translation;

        // Player bounds in horizontal direction
        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max
        }
        // Player bounds in vertical direction
        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        println!("{:?} Should stop at {:?}", translation, (window.width(), window.height()));
    }
}
*/