use traffic_agents::TripPurpose;
use traffic_world::GridPos;

#[derive(Debug, Clone)]
pub struct ActiveVehicle {
    pub agent_id: String,
    pub route: Vec<GridPos>,
    pub route_index: usize,
    pub progress: f32,
    pub speed_tiles_per_sec: f32,
    pub purpose: TripPurpose,
    pub wait_timer_secs: f32,
}

impl ActiveVehicle {
    pub fn new(
        agent_id: String,
        route: Vec<GridPos>,
        purpose: TripPurpose,
        speed_tiles_per_sec: f32,
    ) -> Self {
        Self {
            agent_id,
            route,
            route_index: 0,
            progress: 0.0,
            speed_tiles_per_sec,
            purpose,
            wait_timer_secs: 0.0,
        }
    }

    pub fn position(&self) -> GridPos {
        self.route.get(self.route_index).copied().unwrap_or(GridPos::new(0, 0))
    }

    pub fn next_node(&self) -> Option<GridPos> {
        self.route.get(self.route_index + 1).copied()
    }

    pub fn arrived(&self) -> bool {
        self.route_index + 1 >= self.route.len() && self.progress >= 1.0
    }
}

pub struct VehicleSim;

impl VehicleSim {
    pub fn advance(vehicle: &mut ActiveVehicle, dt: f32, can_enter_next: bool) -> bool {
        if vehicle.arrived() {
            return true;
        }
        if !can_enter_next {
            vehicle.wait_timer_secs += dt;
            return false;
        }
        vehicle.wait_timer_secs = 0.0;
        vehicle.progress += vehicle.speed_tiles_per_sec * dt;
        while vehicle.progress >= 1.0 && vehicle.route_index + 1 < vehicle.route.len() {
            vehicle.progress -= 1.0;
            vehicle.route_index += 1;
        }
        vehicle.arrived()
    }
}