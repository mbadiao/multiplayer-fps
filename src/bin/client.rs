use bevy::audio::AudioPlugin;
use bevy::{app::AppExit, window::WindowCloseRequested};
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
        .add_systems(Last, handle_window_close_requested)
        .run();
}



fn handle_window_close_requested(
    mut exit: EventWriter<AppExit>,
    mut window_close_events: EventReader<WindowCloseRequested>,
    network_resource: Res<NetworkResource>,
) {
    for _ in window_close_events.read() {
        send_quit_message(&network_resource);
        exit.send(AppExit::Success);
    }
}

fn send_quit_message(network_resource: &Res<NetworkResource>) {
    let socket = network_resource.socket.clone();
    let encode = bincode::serialize(&Message::Leave).unwrap();
    
    // Create a new runtime for the blocking context
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        socket.send_to(&encode, socket.peer_addr().unwrap()).await
    });
    
    match result {
        Ok(_) => println!("Quit message sent successfully"),
        Err(e) => eprintln!("Failed to send quit message: {}", e),
    }
    
    // Give some time for the message to be sent
    std::thread::sleep(std::time::Duration::from_millis(200));
}