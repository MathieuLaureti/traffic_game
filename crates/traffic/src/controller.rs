use serde::{Deserialize, Serialize};
use traffic_world::{Direction, GridPos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControllerKind {
    AllWayStop,
    TrafficLight,
    Roundabout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightPhase {
    NorthSouthGreen,
    EastWestGreen,
}

#[derive(Debug, Clone)]
pub struct IntersectionController {
    pub pos: GridPos,
    pub kind: ControllerKind,
    pub light_phase: LightPhase,
    pub phase_elapsed_secs: f32,
    pub green_ns_secs: f32,
    pub green_ew_secs: f32,
    pub stop_hold_secs: f32,
    pub ring_occupied: bool,
}

impl IntersectionController {
    pub fn new(pos: GridPos, kind: ControllerKind) -> Self {
        Self {
            pos,
            kind,
            light_phase: LightPhase::NorthSouthGreen,
            phase_elapsed_secs: 0.0,
            green_ns_secs: 8.0,
            green_ew_secs: 8.0,
            stop_hold_secs: 1.0,
            ring_occupied: false,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.kind != ControllerKind::TrafficLight {
            return;
        }
        self.phase_elapsed_secs += dt;
        let limit = match self.light_phase {
            LightPhase::NorthSouthGreen => self.green_ns_secs,
            LightPhase::EastWestGreen => self.green_ew_secs,
        };
        if self.phase_elapsed_secs >= limit {
            self.phase_elapsed_secs = 0.0;
            self.light_phase = match self.light_phase {
                LightPhase::NorthSouthGreen => LightPhase::EastWestGreen,
                LightPhase::EastWestGreen => LightPhase::NorthSouthGreen,
            };
        }
    }

    pub fn approach_direction(from: GridPos, to: GridPos) -> Option<Direction> {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        match (dx, dy) {
            (0, 1) => Some(Direction::North),
            (1, 0) => Some(Direction::East),
            (0, -1) => Some(Direction::South),
            (-1, 0) => Some(Direction::West),
            _ => None,
        }
    }

    pub fn allows_entry(
        &self,
        from: GridPos,
        to: GridPos,
        waited_secs: f32,
        vehicle_on_ring: bool,
    ) -> bool {
        match self.kind {
            ControllerKind::AllWayStop => waited_secs >= self.stop_hold_secs,
            ControllerKind::TrafficLight => {
                let Some(dir) = Self::approach_direction(from, to) else {
                    return true;
                };
                match self.light_phase {
                    LightPhase::NorthSouthGreen => dir.is_ns(),
                    LightPhase::EastWestGreen => !dir.is_ns(),
                }
            }
            ControllerKind::Roundabout => !vehicle_on_ring && !self.ring_occupied,
        }
    }
}

pub fn parse_controller(s: &str) -> ControllerKind {
    match s.to_lowercase().as_str() {
        "light" | "traffic_light" => ControllerKind::TrafficLight,
        "roundabout" => ControllerKind::Roundabout,
        _ => ControllerKind::AllWayStop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_requires_hold_time() {
        let ctrl = IntersectionController::new(GridPos::new(0, 0), ControllerKind::AllWayStop);
        assert!(!ctrl.allows_entry(
            GridPos::new(0, -1),
            GridPos::new(0, 0),
            0.0,
            false,
        ));
        assert!(ctrl.allows_entry(
            GridPos::new(0, -1),
            GridPos::new(0, 0),
            1.5,
            false,
        ));
    }
}