use serde::{Deserialize, Serialize};

pub const NEEDS: usize = 3;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NeedBars {
    pub food: f32,
    pub entertainment: f32,
    pub necessities: f32,
}

impl Default for NeedBars {
    fn default() -> Self {
        Self {
            food: 0.85,
            entertainment: 0.85,
            necessities: 0.85,
        }
    }
}

impl NeedBars {
    pub const COMFORT: f32 = 0.45;
    pub const CRITICAL: f32 = 0.25;

    pub fn decay(&mut self, minutes: f32) {
        let factor = minutes / 60.0;
        self.food = (self.food - 0.08 * factor).clamp(0.0, 1.0);
        self.entertainment = (self.entertainment - 0.04 * factor).clamp(0.0, 1.0);
        self.necessities = (self.necessities - 0.06 * factor).clamp(0.0, 1.0);
    }

    pub fn fill(&mut self, need: traffic_world::NeedKind, amount: f32) {
        match need {
            traffic_world::NeedKind::Food => self.food = (self.food + amount).min(1.0),
            traffic_world::NeedKind::Entertainment => {
                self.entertainment = (self.entertainment + amount).min(1.0)
            }
            traffic_world::NeedKind::Necessities => {
                self.necessities = (self.necessities + amount).min(1.0)
            }
        }
    }

    pub fn lowest_need(&self) -> Option<traffic_world::NeedKind> {
        let mut items = [
            (traffic_world::NeedKind::Food, self.food),
            (traffic_world::NeedKind::Entertainment, self.entertainment),
            (traffic_world::NeedKind::Necessities, self.necessities),
        ];
        items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        if items[0].1 < Self::COMFORT {
            Some(items[0].0)
        } else {
            None
        }
    }

    pub fn is_critical(&self) -> bool {
        self.food < Self::CRITICAL
            || self.entertainment < Self::CRITICAL
            || self.necessities < Self::CRITICAL
    }
}