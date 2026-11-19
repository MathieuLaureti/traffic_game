//! In-game time: minutes from midnight, pause, and speed multiplier.

use serde::{Deserialize, Serialize};

/// Minutes in one in-game day (1440 = 24h).
pub const MINUTES_PER_DAY: u32 = 24 * 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimClock {
    /// Current time as minutes since midnight `[0, MINUTES_PER_DAY)`.
    pub minute_of_day: u32,
    /// Real seconds for one full in-game day at `time_scale == 1.0`.
    pub day_length_secs: f32,
    pub paused: bool,
    pub time_scale: f32,
}

impl Default for SimClock {
    fn default() -> Self {
        Self {
            minute_of_day: 6 * 60,
            day_length_secs: 600.0,
            paused: false,
            time_scale: 1.0,
        }
    }
}

impl SimClock {
    pub fn advance(&mut self, real_dt: f32) {
        if self.paused {
            return;
        }
        let in_game_minutes_per_real_sec = MINUTES_PER_DAY as f32 / self.day_length_secs;
        let delta_minutes = real_dt * self.time_scale * in_game_minutes_per_real_sec;
        let wrapped = (self.minute_of_day as f32 + delta_minutes) % MINUTES_PER_DAY as f32;
        self.minute_of_day = wrapped as u32;
    }

    pub fn hour(&self) -> u32 {
        self.minute_of_day / 60
    }

    pub fn minute(&self) -> u32 {
        self.minute_of_day % 60
    }

    pub fn format_hm(&self) -> String {
        format!("{:02}:{:02}", self.hour(), self.minute())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScheduleProfile {
    pub id: &'static str,
    pub work_start_minute: u32,
    pub work_end_minute: u32,
}

impl ScheduleProfile {
    pub const STANDARD_9_5: Self = Self {
        id: "standard_9_5",
        work_start_minute: 9 * 60,
        work_end_minute: 17 * 60,
    };

    pub const EARLY_SHIFT: Self = Self {
        id: "early_shift",
        work_start_minute: 6 * 60,
        work_end_minute: 14 * 60,
    };

    pub const LATE_SHIFT: Self = Self {
        id: "late_shift",
        work_start_minute: 14 * 60,
        work_end_minute: 22 * 60,
    };

    pub fn commute_depart_target(&self, commute_buffer_min: u32, jitter: i32) -> u32 {
        let base = self.work_start_minute.saturating_sub(commute_buffer_min);
        let with_jitter = base as i32 + jitter;
        with_jitter.clamp(0, MINUTES_PER_DAY as i32 - 1) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_wraps_at_midnight() {
        let mut clock = SimClock {
            minute_of_day: MINUTES_PER_DAY - 1,
            day_length_secs: 60.0,
            paused: false,
            time_scale: 1.0,
        };
        clock.advance(2.0);
        assert!(clock.minute_of_day < 120);
    }
}