use bevy::prelude::*;
use bevy_rapier3d::prelude::{CollidingEntities, CollisionEvent, ContactForceEvent, Damping, ExternalForce, LockedAxes};
use crate::client::components::player_component::{Player, Velocity};

// pub fn collider_detect_world(
//     mut collision_events: EventReader<CollisionEvent>,
//     mut _query: Query<(&mut Velocity, &Transform)>
// ) {
//     for collision_event in collision_events.read() {
//         match collision_event {
//             CollisionEvent::Started(e1, e2, flags) => {
//                 // if let Ok((mut velocity, transform, _)) = query.get_mut(*e1){}
//                 println!("Collision started between entities {:?} and {:?} -- {:?}", e1, e2,flags);
//             }
//             CollisionEvent::Stopped(e1, e2, flags) => {
//                 println!("Collision stopped between entities {:?} and {:?} -- {:?}", e1, e2,flags);
//             }
//         }
//     }
// }


pub fn collider_detect_world(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Velocity, &mut ExternalForce, &mut Damping)>,
) {
    for collision_event in collision_events.read() {
        println!("Collision detected: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, _, _) => {
                if let Ok((mut velocity, mut force, mut damping)) = query.get_mut(*e1) {
                    velocity.0 = Vec3::ZERO;
                    force.force = Vec3::ZERO;
                    damping.linear_damping = 10.0; // Increase damping when colliding
                }
            },
            CollisionEvent::Stopped(e1, _, _) => {
                if let Ok((_, _, mut damping)) = query.get_mut(*e1) {
                    damping.linear_damping = 1.0; // Restore normal movement
                }
            }
        }
    }
}

pub fn debug_collisions(query: Query<&CollidingEntities, With<Player>>) {
    for colliding in query.iter() {
        if colliding.is_empty() {
            println!("Le joueur ne touche rien.");
        } else {
            for entity in colliding.iter() {
                println!("Collision avec l'entité {:?}", entity);
            }
        }
    }
}