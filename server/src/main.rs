use bevy::prelude::*;

mod server_plugin;

use crate::server_plugin::ServerPlugin;

fn main() -> AppExit {
    App::new().add_plugins(ServerPlugin).run()
}
