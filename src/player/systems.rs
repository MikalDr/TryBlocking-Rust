use bevy::{prelude::*, window::PrimaryWindow, input::keyboard};
use bevy_rapier2d::prelude::*;

use super::{component::Player, PLAYER_GROUNDED};

const SPRITE_PATH: &str = "TemPlayer.png";
pub const PLAYER_SPEED: f32 = 350.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const MAX_GRAVITY_SCALE: f32 = 300.0;
pub const GRAVITY_SCALE: f32 = 14.0;

pub static mut PLAYER_VERTICAL: f32 = 0.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands
    .spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0 , window.height()/2.0, 0.0),
                texture: asset_server.load(SPRITE_PATH),
                ..default()
            },
            Player {}
        ),
    )
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(10.0))
    .insert(KinematicCharacterController {
        up: Vec2::X,
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    is_player_grounded: Res<PLAYER_GROUNDED>,
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
    mut character_controller_output: Query<&mut KinematicCharacterControllerOutput, With<Player>>
)
{
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec2::ZERO;
        
        if unsafe { PLAYER_VERTICAL } <= MAX_GRAVITY_SCALE {
            unsafe { PLAYER_VERTICAL += GRAVITY_SCALE };
        }

        let mut gravity = Vec2::new(0.0, -unsafe { PLAYER_VERTICAL });

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec2::new(-1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec2::new(1.0, 0.0)
        }
        
        // Jump Function
        if keyboard_input.just_pressed(KeyCode::Space) {
            unsafe { PLAYER_VERTICAL = -230.0 };
        }

        for mut controller in controllers.iter_mut() {
            controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds() + gravity * time.delta_seconds());
        }

        //Gravity controller
    }
}