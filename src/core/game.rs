use bevy::prelude::*;

use crate::{
    core::{camera::CameraPlugin, screen::ScreenPlugin},
    physics::{gravity::GravityPlugin, velocity::VelocityPlugin},
};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins;

        let screen_plugin = ScreenPlugin;
        let camera_plugin = CameraPlugin;

        app.add_plugins(default_plugins)
            .add_plugins(screen_plugin)
            .add_plugins(VelocityPlugin)
            .add_plugins(GravityPlugin)
            .add_plugins(camera_plugin);
    }
}
