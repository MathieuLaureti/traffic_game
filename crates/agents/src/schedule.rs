use traffic_sim_time::ScheduleProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScheduleKind {
    Standard95,
    Early,
    Late,
}

impl ScheduleKind {
    pub fn from_str_id(id: &str) -> Self {
        match id {
            "early_shift" => ScheduleKind::Early,
            "late_shift" => ScheduleKind::Late,
            _ => ScheduleKind::Standard95,
        }
    }

    pub fn profile(self) -> ScheduleProfile {
        match self {
            ScheduleKind::Standard95 => ScheduleProfile::STANDARD_9_5,
            ScheduleKind::Early => ScheduleProfile::EARLY_SHIFT,
            ScheduleKind::Late => ScheduleProfile::LATE_SHIFT,
        }
    }
}

pub fn planned_commute_depart(
    kind: ScheduleKind,
    commute_buffer_minutes: u32,
    jitter_minutes: i32,
) -> u32 {
    kind.profile().commute_depart_target(commute_buffer_minutes, jitter_minutes)
}