use bevy::prelude::*;

use crate::sim::SimulationState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (keyboard_controls,));
    }
}

fn keyboard_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut sim: ResMut<SimulationState>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::Space) {
        sim.clock.paused = !sim.clock.paused;
    }
    if keys.just_pressed(KeyCode::Digit1) {
        sim.clock.time_scale = 1.0;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        sim.clock.time_scale = 2.0;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        sim.clock.time_scale = 4.0;
    }
}