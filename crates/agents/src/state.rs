use crate::NeedBars;
use crate::ScheduleKind;
use serde::{Deserialize, Serialize};
use traffic_world::GridPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    AtHome,
    Commuting,
    AtWork,
    Consuming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TripPurpose {
    CommuteToWork,
    CommuteHome,
    Consume(traffic_world::NeedKind),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub home: GridPos,
    pub workplace: GridPos,
    pub schedule: ScheduleKind,
    pub jitter_minutes: i32,
    pub state: AgentState,
    pub needs: NeedBars,
    pub location: GridPos,
    /// Minute-of-day when agent should leave home for work (computed at day start).
    pub morning_depart_minute: u32,
    pub commuted_to_work_today: bool,
    pub returned_home_today: bool,
}

impl Agent {
    pub fn at_home_idling(&self) -> bool {
        self.state == AgentState::AtHome && self.location == self.home
    }

    pub fn at_work(&self) -> bool {
        self.state == AgentState::AtWork
    }
}