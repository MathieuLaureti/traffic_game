# Delivery: MVP foundation

## Status

pr_open

## Summary

Rust workspace with six crates plus Bevy shell, demo scenario, and core sim loop from the design input doc.

## Issues

- Closes #1

## Verification

| Check | Result |
|-------|--------|
| `cargo test` | pass (sim_time, pathing, world, traffic) |
| `cargo build -p traffic_game` | pass |
| Manual run | `cargo run -p traffic_game` from repo root |

## UI / behavior notes

- Scenario-authored POIs; player controls pause/speed only in this slice.
- Morning commute departures; stop / light / roundabout controllers on demo intersections.
- Player signal editing and win metrics: follow-up issues.