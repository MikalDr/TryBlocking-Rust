use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::component::Player;

mod systems;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera {
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_camera)
        .add_systems(Update, camera_follow_player);
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window = window_query.get_single().unwrap();

    commands.spawn((Camera2dBundle 
        {
            transform: Transform::from_xyz(window.width()/2.0 , window.height()/2.0, 0.0),
            projection: OrthographicProjection {
                scale: 1.0,
                ..default()
            },
            ..default()
        },
        MainCamera {}
    ));
}

pub fn camera_follow_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<(&mut Camera, &mut Transform), Without<Player>>,
) 
{
    if let Ok((mut camera, mut camera_transform)) = camera_query.get_single_mut(){
        if let Ok(transform) = player_query.get_single_mut(){
            camera_transform.translation = transform.translation;
        }

        println!("{}", camera_transform.translation);
    }
}