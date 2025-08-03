use bevy::prelude::*;

use crate::core::game_plugin::GamePlugin;
mod core;

fn main() {

    App::new()
        .add_plugins(GamePlugin)
        .run();

}
