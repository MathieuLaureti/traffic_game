use crate::{GridPos, IntersectionControlDef, Poi, TileKind, WorldMap};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScenarioError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid scenario: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpawn {
    pub id: String,
    pub home_poi: String,
    pub workplace_poi: String,
    pub schedule_profile: String,
    pub jitter_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioConfig {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<String>,
    pub pois: Vec<Poi>,
    pub intersections: Vec<IntersectionControlDef>,
    pub agents: Vec<AgentSpawn>,
    pub commute_buffer_minutes: u32,
    pub seed: u64,
}

impl ScenarioConfig {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ScenarioError> {
        let text = fs::read_to_string(path)?;
        Self::from_str(&text)
    }

    pub fn from_str(text: &str) -> Result<Self, ScenarioError> {
        serde_json::from_str(text).map_err(ScenarioError::from)
    }

    pub fn build_map(&self) -> Result<WorldMap, ScenarioError> {
        let expected = (self.width * self.height) as usize;
        let tile_chars: usize = self.tiles.iter().map(|r| r.len()).sum();
        if self.tiles.len() as u32 != self.height || tile_chars != expected {
            return Err(ScenarioError::Invalid(format!(
                "tile grid {} rows / {} chars != {}x{}",
                self.tiles.len(),
                tile_chars,
                self.width,
                self.height
            )));
        }
        let tiles: Vec<TileKind> = self
            .tiles
            .iter()
            .flat_map(|row| row.chars())
            .map(|c| match c {
                '.' => TileKind::Void,
                'R' => TileKind::Road,
                'I' => TileKind::Intersection,
                'B' => TileKind::Building,
                _ => TileKind::Void,
            })
            .collect();

        Ok(WorldMap {
            width: self.width,
            height: self.height,
            tiles,
            intersections: self.intersections.clone(),
        })
    }

    pub fn poi_by_id(&self, id: &str) -> Option<&Poi> {
        self.pois.iter().find(|p| p.id == id)
    }

    pub fn nearest_service(&self, from: GridPos, variant: crate::ServiceVariant) -> Option<&Poi> {
        self.pois
            .iter()
            .filter(|p| {
                matches!(
                    p.kind,
                    crate::PoiKind::ServiceTier2 { variant: v, .. } if v == variant
                )
            })
            .min_by_key(|p| manhattan(from, p.access_road))
    }
}

fn manhattan(a: GridPos, b: GridPos) -> u32 {
    ((a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_scenario_loads() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/scenarios/demo.json");
        let cfg = ScenarioConfig::from_path(path).expect("demo json");
        let map = cfg.build_map().expect("map");
        assert_eq!(map.width, 15);
        assert!(!cfg.agents.is_empty());
    }
}