use bevy::{prelude::*, window::PrimaryWindow, input::keyboard};

const SPRITE_PATH: &str = "TemPlayer.png";


#[derive(Component)]
pub struct Player {}

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
        )
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
)
{
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.just_pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
    }
}