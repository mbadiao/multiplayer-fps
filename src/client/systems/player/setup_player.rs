use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const INITIAL_POSITION_PLAYER: Vec3 = Vec3::new(20., 0.6, 20.);

use crate::client::{
    components::{
        camera_component::CameraSensitivity,
        player_component::{
            AccumulatedInput, PhysicalTranslation, Player, PreviousPhysicalTranslation, Velocity,
        },
    },
    systems::camera::{
        view_model_camera::spawn_view_model_camera, world_model_camera::spawn_main_camera,
    },
};

use super::view_model_player::spawn_view_model;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    camera_sensitivity: CameraSensitivity,
    accumulated_input: AccumulatedInput,
    pub velocity: Velocity,
    physical_translation: PhysicalTranslation,
    previous_physical_translation: PreviousPhysicalTranslation,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    collider: Collider,
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    damping: Damping,
    friction: Friction,
    restitution: Restitution,
}

// Le player instancie les camera comme enfant

/*
    Le Player est une entité qui représente l'état du joueur
     (position, rotation, etc.) et sert de conteneur pour
      d'autres composants comme les caméras.
*/
// setup_player
fn spawn_player(commands: &mut Commands) -> Entity {
    commands
        .spawn(PlayerBundle {
            player: Player,
            camera_sensitivity: CameraSensitivity::default(),
            accumulated_input: AccumulatedInput(Vec2::ZERO),
            velocity: Velocity(Vec3::ZERO),
            physical_translation: PhysicalTranslation(INITIAL_POSITION_PLAYER),
            previous_physical_translation: PreviousPhysicalTranslation(INITIAL_POSITION_PLAYER),
            transform: Transform::from_xyz(
                INITIAL_POSITION_PLAYER.x,
                INITIAL_POSITION_PLAYER.y,
                INITIAL_POSITION_PLAYER.z,
            ),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            collider: Collider::cuboid(0.01, 0.01, 0.01),
            // collider: Collider::ball(0.1),
            rigid_body: RigidBody::Dynamic,
            gravity_scale: GravityScale(0.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        })
        .id()
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = spawn_player(&mut commands);

    commands.entity(player).with_children(|parent| {
        spawn_main_camera(parent);
        spawn_view_model_camera(parent);
        spawn_view_model(parent, &mut meshes, &mut materials);
    });
}
