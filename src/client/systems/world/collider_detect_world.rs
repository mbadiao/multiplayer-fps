use crate::client::components::player_component::Velocity;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{CollisionEvent, ContactForceEvent};

pub fn collider_detect_world(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Velocity, &Transform)>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, flags) => {
                  // Handle both entities in collision
                  if let Ok((mut velocity1, _)) = query.get_mut(*e1) {
                    // Stop all movement
                    velocity1.x = 0.0;
                    velocity1.y = 0.0;
                    velocity1.z = 0.0;
                }
                
                 println!("Collision started between entities {:?} and {:?} -- {:?}", e1, e2,flags);
            }
            CollisionEvent::Stopped(e1, e2, flags) => {
                
                // Reset velocity when collision stops
                if let Ok((mut velocity, _)) = query.get_mut(*e1) {
                    velocity.x = 0.;
                    velocity.y = 0.;
                    velocity.z = 0.;
                }
                if let Ok((mut velocity, _)) = query.get_mut(*e2) {
                    velocity.x = 0.;
                    velocity.y = 0.;
                    velocity.z = 0.;
                }
                println!(
                    "Collision stopped between entities {:?} and {:?} -- {:?}",
                    e1, e2, flags
                );
            }
        }
    }
}
