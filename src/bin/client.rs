use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use multiplayer_fps::{
    client::{
        plugins::{player_plugin::PlayerPlugin, world_plugin::WorldPlugin},
        resources::network_resource::{handle_network_messages, input_connexion, NetworkResource},
        udp::Client,
    },
    common::network::protocol::Message,
};
use std::sync::Arc;
use tokio::{net::UdpSocket, runtime::Runtime};

fn main() {
    // Créer le runtime une seule fois
    let runtime = Runtime::new().unwrap();

    let (name, remote_addr) = input_connexion();

    // Établir la connexion et obtenir le socket
    let socket = runtime.block_on(async {
        let client = Client::new(name);
        match client.connect(remote_addr).await {
            Ok(so) => so,
            Err(e) => panic!("Error: {}", e),
        }
    });

    // Une fois connecté, démarrer Bevy
    App::new()
        .add_plugins((DefaultPlugins, WorldPlugin, PlayerPlugin))
        .insert_resource(NetworkResource {
            socket: Arc::new(socket),
        })
        .add_systems(Update, handle_network_messages)
        .add_systems(Update, handle_app_exit)
        .run();
}

fn handle_app_exit(mut exit_events: EventReader<AppExit>, network_ressource: Res<NetworkResource>) {
    for _ in exit_events.read() {
        let socket = &network_ressource.socket;
        let encode = bincode::serialize(&Message::Leave).unwrap();
        let runtime = Runtime::new().unwrap();
        runtime.block_on(send_quit_message(socket, encode));
    }
}

async fn send_quit_message(socket: &Arc<UdpSocket>, encode: Vec<u8>) {
    if let Err(e) = socket.send(&encode).await {
        eprint!("{e}");
    };
}
