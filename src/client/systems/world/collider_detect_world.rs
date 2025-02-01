use bevy::prelude::*;
use bevy_rapier3d::prelude::{CollisionEvent, ContactForceEvent, ExternalForce, LockedAxes};
use crate::client::components::player_component::Velocity;

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
    mut query: Query<(&mut Velocity, &mut ExternalForce, &mut LockedAxes)>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok((mut velocity, mut force, mut locked_axes)) = query.get_mut(*e1) {
                    velocity.0 = Vec3::ZERO;
                    force.force = Vec3::ZERO;
                    *locked_axes = LockedAxes::TRANSLATION_LOCKED;
                }
            }
            CollisionEvent::Stopped(e1, _, _) => {
                if let Ok((_, _, mut locked_axes)) = query.get_mut(*e1) {
                    *locked_axes = LockedAxes::ROTATION_LOCKED; // Re-enable movement if needed
                }
            }
        }
    }
}
