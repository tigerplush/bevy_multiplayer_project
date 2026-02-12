use bevy::prelude::*;

mod client_plugin;
mod player;

use client_plugin::ClientPlugin;

fn main() -> AppExit {
    App::new().add_plugins(ClientPlugin).run()
}
