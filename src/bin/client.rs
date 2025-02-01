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

fn handle_app_exit(mut exit_events: EventReader<AppExit>, network_resource: Res<NetworkResource>) {
    for _ in exit_events.read() {
        println!("Shutting down client..."); // Better logging
        let socket = network_resource.socket.clone();
        let encode = bincode::serialize(&Message::Leave).unwrap();

        // Use blocking to ensure message is sent
        std::thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            if let Err(e) = runtime.block_on(send_quit_message(&socket, encode)) {
                eprintln!("Failed to send quit message: {}", e);
            }
            // Add small delay to ensure message is sent
            std::thread::sleep(std::time::Duration::from_millis(100));
        })
        .join()
        .unwrap();
    }
}

async fn send_quit_message(socket: &Arc<UdpSocket>, encode: Vec<u8>) -> Result<(), std::io::Error> {
    socket.send(&encode).await?;
    Ok(())
}
