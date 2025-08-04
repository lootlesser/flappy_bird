use bevy::prelude::*;

use crate::core::game_plugin::GamePlugin;
mod core;
mod physics;

fn main() {

    App::new()
        .add_plugins(GamePlugin)
        .run();

}
