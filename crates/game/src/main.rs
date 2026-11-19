mod plugins;
mod sim;

use bevy::prelude::*;
use plugins::{GameUiPlugin, InputPlugin, RenderPlugin, SimPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Traffic Game".into(),
                resolution: (1280_f32, 720_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((SimPlugin, RenderPlugin, GameUiPlugin, InputPlugin))
        .run();
}