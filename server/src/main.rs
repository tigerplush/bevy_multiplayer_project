use bevy::prelude::*;

mod movement;
mod player;
mod server_plugin;

use crate::server_plugin::ServerPlugin;

fn main() -> AppExit {
    App::new().add_plugins(ServerPlugin).run()
}
