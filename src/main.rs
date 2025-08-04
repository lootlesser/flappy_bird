use bevy::prelude::*;

use crate::core::game::GamePlugin;
mod core;
mod physics;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
