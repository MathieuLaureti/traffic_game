use traffic_agents::Agent;
use traffic_sim_time::SimClock;
use traffic_traffic::{IntersectionController, TrafficMetrics};
use traffic_world::{RoadGraph, ScenarioConfig, WorldMap};

#[derive(bevy::prelude::Resource)]
pub struct SimulationState {
    pub scenario: ScenarioConfig,
    pub map: WorldMap,
    pub graph: RoadGraph,
    pub clock: SimClock,
    pub agents: Vec<Agent>,
    pub controllers: Vec<IntersectionController>,
    pub vehicles: Vec<traffic_traffic::ActiveVehicle>,
    pub metrics: TrafficMetrics,
    pub tile_size: f32,
}

impl SimulationState {
    pub fn load_default() -> Self {
        let path = "assets/scenarios/demo.json";
        let scenario = ScenarioConfig::from_path(path).expect("load demo scenario");
        let map = scenario.build_map().expect("build map");
        let graph = RoadGraph::from_map(&map);
        let agents = traffic_agents::spawn_agents(&scenario);
        let controllers = scenario
            .intersections
            .iter()
            .map(|ic| {
                IntersectionController::new(
                    ic.pos,
                    traffic_traffic::parse_controller(&ic.controller),
                )
            })
            .collect();

        Self {
            scenario,
            map,
            graph,
            clock: SimClock::default(),
            agents,
            controllers,
            vehicles: Vec::new(),
            metrics: TrafficMetrics::default(),
            tile_size: 32.0,
        }
    }
}