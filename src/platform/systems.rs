use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;


pub fn spawn_platform(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(window.width()/2.0, 0.0, 0.0),
                    ..default()
                }, 
                RigidBody::Fixed,
                Collider::cuboid(window.width()/2.0, 20.0),
                ActiveEvents::COLLISION_EVENTS,
            ),
    );
}

/* A system that displays the events. */
pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}