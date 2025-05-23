use bevy::prelude::*;

use crate::client::systems::player::{
    mini_map_player::update_minimap_player,
    move_player::move_player,
    rotate_player::rotate_player,
    send_update_player::send_player_updates,
    setup_player::setup,
    shooting::{handle_bullet_collision, handle_wall_collision, player_shooting, update_bullets},
    step::{setupsoundshoot, setupsoundsprint},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setupsoundsprint, setupsoundshoot)) // setup le player revient a setup les cameras
            .add_systems(
                Update,
                (
                    rotate_player,
                    move_player,
                    player_shooting,
                    update_bullets,
                    handle_wall_collision,
                    update_minimap_player,
                    send_player_updates,
                ),
            )
            .add_systems(
                Update,
                (handle_bullet_collision, send_player_updates).chain(),
            );
    }
}
