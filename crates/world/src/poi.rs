use crate::GridPos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeedKind {
    Food,
    Entertainment,
    Necessities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceVariant {
    Food,
    Entertainment,
    Necessities,
}

impl ServiceVariant {
    pub fn need_kind(self) -> NeedKind {
        match self {
            ServiceVariant::Food => NeedKind::Food,
            ServiceVariant::Entertainment => NeedKind::Entertainment,
            ServiceVariant::Necessities => NeedKind::Necessities,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PoiKind {
    Living,
    JobTier3 { schedule_profile: String },
    ServiceTier2 {
        variant: ServiceVariant,
        schedule_profile: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poi {
    pub id: String,
    pub position: GridPos,
    pub access_road: GridPos,
    pub kind: PoiKind,
}