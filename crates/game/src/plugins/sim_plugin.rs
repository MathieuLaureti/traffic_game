use bevy::prelude::*;

use crate::sim::{advance_simulation, SimulationState};

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationState::load_default())
            .add_systems(Update, run_simulation);
    }
}

fn run_simulation(time: Res<Time>, mut state: ResMut<SimulationState>) {
    advance_simulation(&mut state, time.delta_secs());
}