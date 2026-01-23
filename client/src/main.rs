use bevy::prelude::*;

mod client_plugin;

use client_plugin::ClientPlugin;

fn main() -> AppExit {
    App::new().add_plugins(ClientPlugin).run()
}
