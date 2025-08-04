use bevy::prelude::*;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, _app: &mut App) {
        let window = Window {
            title: "Flappy-Bird".to_string(),
            ..Default::default()
        };

        let window_plugin = WindowPlugin {
            primary_window: Some(window),
            close_when_requested: true,
            exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        };

        DefaultPlugins.set(window_plugin);
    }
}
