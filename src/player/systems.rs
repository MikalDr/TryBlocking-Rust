use bevy::{prelude::*, window::PrimaryWindow, input::keyboard};
use bevy_rapier2d::prelude::*;

use super::{component::Player, PLAYER_RESOURCES};

const SPRITE_PATH: &str = "TemPlayer.png";
pub const PLAYER_SPEED: f32 = 350.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const MAX_GRAVITY_SCALE: f32 = 300.0;
pub const JUMP_FORCE: f32 = 300.0;
pub const GRAVITY_SCALE: f32 = 10.0;
pub const AIRTIME_DELAY: f32 = 0.05;
pub const COYOTE_TIME: f32 = 0.05;

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
    .insert(KinematicCharacterController::default())
    .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
    ;
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut player_resources: ResMut<PLAYER_RESOURCES>,
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
)
{
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec2::ZERO;
        if player_resources.player_vertical <= MAX_GRAVITY_SCALE {
            player_resources.player_vertical += GRAVITY_SCALE ;
        }

        let mut gravity = Vec2::new(0.0, -player_resources.player_vertical);
        
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec2::new(-1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec2::new(1.0, 0.0)
        }
        
        // Jump Function
        if keyboard_input.just_pressed(KeyCode::Space) {
            player_resources.air_timer = AIRTIME_DELAY;
            if player_resources.is_ground || player_resources.coyote_timer > 0.0{
                    player_resources.jumped = true;
                    player_resources.player_vertical = -JUMP_FORCE;
                    player_resources.air_timer = 0.0;
                    player_resources.coyote_timer = 0.0;
            }
        }
        //Jump time error
        if (player_resources.air_timer > 0.0  && player_resources.is_ground){
            player_resources.jumped = true;
            player_resources.player_vertical = -JUMP_FORCE;
            player_resources.air_timer = 0.0;
            player_resources.coyote_timer = 0.0;
        }

        //Timers
        if(player_resources.air_timer > 0.0){
            player_resources.air_timer -= time.delta_seconds();
        }
        if(player_resources.coyote_timer > 0.0){
            player_resources.coyote_timer -= time.delta_seconds();
        }
        //PlayerUpdate
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(direction * PLAYER_SPEED * time.delta_seconds() + gravity * time.delta_seconds());
        }
        println!("{:?}", player_resources.player_vertical)

        //Gravity controller
    }
}

pub fn player_ground_check(mut player_resources: ResMut<PLAYER_RESOURCES>,
    controller_output: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    for (_entity, output) in controller_output.iter() {
        match output.grounded {
            true => {
                player_resources.is_ground = true;
                player_resources.jumped = false;
                player_resources.falling = false;
            }
            false => {
                player_resources.is_ground = false;
                if !player_resources.jumped && player_resources.coyote_timer <= 0.0 {
                    player_resources.coyote_timer = COYOTE_TIME;
                }
            }
        }
    }
}