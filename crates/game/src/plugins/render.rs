use bevy::prelude::*;
use traffic_world::TileKind;

use crate::sim::SimulationState;

#[derive(Component)]
struct MapTile;

#[derive(Component)]
struct VehicleSprite;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, build_map))
            .add_systems(Update, (sync_vehicles,));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn build_map(mut commands: Commands, sim: Res<SimulationState>) {
    let ts = sim.tile_size;
    for y in 0..sim.map.height {
        for x in 0..sim.map.width {
            let pos = traffic_world::GridPos::new(x as i32, y as i32);
            let kind = sim.map.get(pos).unwrap_or(TileKind::Void);
            let color = match kind {
                TileKind::Void => Color::srgb(0.12, 0.14, 0.16),
                TileKind::Road => Color::srgb(0.35, 0.35, 0.38),
                TileKind::Intersection => Color::srgb(0.45, 0.42, 0.35),
                TileKind::Building => Color::srgb(0.55, 0.35, 0.25),
            };
            let world = grid_to_world(pos, ts);
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(ts - 1.0)),
                    ..default()
                },
                Transform::from_translation(world.extend(0.0)),
                MapTile,
            ));
        }
    }

    for poi in &sim.scenario.pois {
        let color = match poi.kind {
            traffic_world::PoiKind::Living => Color::srgb(0.3, 0.6, 0.9),
            traffic_world::PoiKind::JobTier3 { .. } => Color::srgb(0.85, 0.75, 0.2),
            traffic_world::PoiKind::ServiceTier2 { .. } => Color::srgb(0.4, 0.85, 0.5),
        };
        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(ts * 0.35)),
                ..default()
            },
            Transform::from_translation(
                grid_to_world(poi.position, ts).extend(2.0),
            ),
            MapTile,
        ));
    }
}

fn sync_vehicles(
    mut commands: Commands,
    sim: Res<SimulationState>,
    q: Query<(Entity, &mut Transform), With<VehicleSprite>>,
) {
    let ts = sim.tile_size;
    let ids: std::collections::HashSet<String> = sim
        .vehicles
        .iter()
        .map(|v| v.agent_id.clone())
        .collect();

    let to_despawn: Vec<Entity> = q.iter().map(|(e, _)| e).collect();
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }

    for vehicle in &sim.vehicles {
        let pos = vehicle.position();
        let mut world = grid_to_world(pos, ts);
        if let Some(next) = vehicle.next_node() {
            let next_w = grid_to_world(next, ts);
            world = world.lerp(next_w, vehicle.progress);
        }
        commands.spawn((
            Sprite {
                color: Color::srgb(0.95, 0.2, 0.2),
                custom_size: Some(Vec2::splat(ts * 0.4)),
                ..default()
            },
            Transform::from_translation(world.extend(5.0)),
            VehicleSprite,
        ));
    }

    let _ = ids;
}

pub fn grid_to_world(pos: traffic_world::GridPos, tile_size: f32) -> Vec2 {
    Vec2::new(pos.x as f32 * tile_size, pos.y as f32 * tile_size)
}