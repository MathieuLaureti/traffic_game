use bevy::prelude::*;

use crate::sim::SimulationState;

#[derive(Component)]
struct HudText;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud).add_systems(Update, refresh_hud);
    }
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            ..default()
        },
        HudText,
    ));
}

fn refresh_hud(sim: Res<SimulationState>, mut q: Query<&mut Text, With<HudText>>) {
    let Ok(mut text) = q.get_single_mut() else {
        return;
    };
    **text = format!(
        "{}  |  scale {:.0}x  |  agents {}  |  vehicles {}  |  trips {}\n[Space] pause  [1/2/3] speed  [Esc] quit",
        sim.clock.format_hm(),
        sim.clock.time_scale,
        sim.agents.len(),
        sim.metrics.vehicles_active,
        sim.metrics.trips_completed,
    );
}