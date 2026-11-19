//! A* on the road graph.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use traffic_world::{GridPos, RoadGraph};

#[derive(Debug, Clone)]
pub struct PathResult {
    pub nodes: Vec<GridPos>,
}

pub fn find_path(graph: &RoadGraph, start: GridPos, goal: GridPos) -> Option<PathResult> {
    if start == goal {
        return Some(PathResult { nodes: vec![start] });
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u32,
        est: u32,
        pos: GridPos,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .est
                .cmp(&self.est)
                .then_with(|| other.cost.cmp(&self.cost))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let h = |a: GridPos, b: GridPos| -> u32 {
        (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
    };

    let mut open = BinaryHeap::new();
    open.push(State {
        cost: 0,
        est: h(start, goal),
        pos: start,
    });

    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    let mut best_cost: HashMap<GridPos, u32> = HashMap::from([(start, 0)]);

    while let Some(State { cost, pos, .. }) = open.pop() {
        if pos == goal {
            let mut nodes = vec![goal];
            let mut cur = goal;
            while let Some(&prev) = came_from.get(&cur) {
                nodes.push(prev);
                cur = prev;
                if cur == start {
                    break;
                }
            }
            nodes.reverse();
            return Some(PathResult { nodes });
        }

        for &next in graph.neighbors(pos) {
            let next_cost = cost + 1;
            if best_cost.get(&next).is_some_and(|&c| c <= next_cost) {
                continue;
            }
            best_cost.insert(next, next_cost);
            came_from.insert(next, pos);
            open.push(State {
                cost: next_cost,
                est: next_cost + h(next, goal),
                pos: next,
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use traffic_world::{TileKind, WorldMap};

    #[test]
    fn straight_line_path() {
        let mut map = WorldMap {
            width: 3,
            height: 1,
            tiles: vec![TileKind::Road, TileKind::Road, TileKind::Road],
            intersections: vec![],
        };
        let _ = &mut map;
        let graph = RoadGraph::from_map(&map);
        let path = find_path(
            &graph,
            GridPos::new(0, 0),
            GridPos::new(2, 0),
        )
        .unwrap();
        assert_eq!(path.nodes.len(), 3);
    }
}