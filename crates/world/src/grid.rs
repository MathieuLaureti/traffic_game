use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbor(self, dir: Direction) -> Self {
        let (dx, dy) = dir.delta();
        Self::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileKind {
    Void,
    Road,
    Intersection,
    Building,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn is_ns(self) -> bool {
        matches!(self, Direction::North | Direction::South)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntersectionControlDef {
    pub pos: GridPos,
    pub controller: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileKind>,
    pub intersections: Vec<IntersectionControlDef>,
}

impl WorldMap {
    pub fn idx(&self, pos: GridPos) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        let (x, y) = (pos.x as u32, pos.y as u32);
        if x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }

    pub fn get(&self, pos: GridPos) -> Option<TileKind> {
        let i = self.idx(pos)?;
        self.tiles.get(i).copied()
    }

    pub fn set(&mut self, pos: GridPos, kind: TileKind) {
        if let Some(i) = self.idx(pos) {
            self.tiles[i] = kind;
        }
    }

    pub fn is_drivable(&self, pos: GridPos) -> bool {
        matches!(
            self.get(pos),
            Some(TileKind::Road) | Some(TileKind::Intersection)
        )
    }
}

#[derive(Debug, Clone)]
pub struct RoadGraph {
    pub adjacency: HashMap<GridPos, Vec<GridPos>>,
}

impl RoadGraph {
    pub fn from_map(map: &WorldMap) -> Self {
        let mut adjacency: HashMap<GridPos, Vec<GridPos>> = HashMap::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let pos = GridPos::new(x as i32, y as i32);
                if !map.is_drivable(pos) {
                    continue;
                }
                let mut neighbors = Vec::new();
                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    let n = pos.neighbor(dir);
                    if map.is_drivable(n) {
                        neighbors.push(n);
                    }
                }
                adjacency.insert(pos, neighbors);
            }
        }
        Self { adjacency }
    }

    pub fn neighbors(&self, pos: GridPos) -> &[GridPos] {
        self.adjacency
            .get(&pos)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}