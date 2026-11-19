# traffic_game

2D top-down grid traffic control game (Rust + Bevy). Minimal city simulation (homes, jobs, tier-2 services) generates commute demand; the player tunes roads and intersection controls.

## Workspace layout

| Crate | Role |
|-------|------|
| `traffic_sim_time` | In-game clock, pause, time scale |
| `traffic_world` | Grid map, POIs, scenario load |
| `traffic_agents` | NPC needs, schedules, trip intent |
| `traffic_pathing` | Grid A* |
| `traffic_traffic` | Vehicles, intersection controllers |
| `traffic_game` | Bevy app, rendering, UI |

Design docs: [`docs/input/2026-11-19-traffic-grid-transport-game.md`](docs/input/2026-11-19-traffic-grid-transport-game.md).

## Run

```bash
cargo run -p traffic_game
```

Controls: **Space** pause · **1 / 2 / 3** time scale · **Esc** quit