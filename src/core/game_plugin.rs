use bevy::prelude::*;

use crate::core::{camera_plugin::CameraPlugin, screen_plugin::ScreenPlugin};
pub struct GamePlugin;

impl Plugin for GamePlugin {

    fn build(&self, app: &mut App) {

        let default_plugins = DefaultPlugins;

        let screen_plugin = ScreenPlugin;
        let camera_plugin = CameraPlugin;

        app
        .add_plugins(default_plugins)
        .add_plugins(screen_plugin)
        .add_plugins(camera_plugin);

    }

}
