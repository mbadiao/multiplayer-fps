use bevy::prelude::*;
use bevy_rapier3d::prelude::{CollisionEvent, ContactForceEvent};
use crate::client::components::player_component::Velocity;

pub fn collider_detect_world(
    mut collision_events: EventReader<CollisionEvent>,
    mut _query: Query<(&mut Velocity, &Transform)>
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, flags) => {
                println!("Collision started between entities {:?} and {:?} -- {:?}", e1, e2,flags);
            }
            CollisionEvent::Stopped(e1, e2, flags) => {
                println!("Collision stopped between entities {:?} and {:?} -- {:?}", e1, e2,flags);
            }
        }
    }
}