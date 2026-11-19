# Traffic grid game — brainstorm

**Status:** promoted → [`../input/2026-11-19-traffic-grid-transport-game.md`](../input/2026-11-19-traffic-grid-transport-game.md)

## Raw intent (from exploration)

- 2D top-down **grid** game in **Rust + Bevy**.
- **Main game = transport / traffic control**, not city-building.
- **Minimal city:** only **living spots** and **job/service spots**.
- Living = where NPCs spend most of their time when not commuting or at activities.
- Roads + classic infrastructure: lights, stop signs, roundabouts, etc.

## City / services model

### Living

- Home tile(s); anchor for OD trips.

### Job spots — two roles

1. **Tier 3 — job only**  
   NPC commutes home ↔ work. No consumption at that location.

2. **Tier 2 — work + consumption**  
   NPCs can be employed there *and* others visit as customers.  
   Three variants, each fills one need bar:
   - Food
   - Entertainment
   - Necessities

## NPC behavior (keep simple)

- Three need bars: food, entertainment, necessities — stay above a comfort level.
- Must **maintain job** (work trips take priority over discretionary shopping when schedules conflict).
- Trip to nearest suitable tier-2 variant when a bar is low (MVP may allow only one need trip at a time).
- Commute should feel **realistic**:
  - **Rush hours** because many jobs share similar hours — especially **9–5** → big morning surge roughly **08:00–10:00** in-game.
  - **Not everyone at once:** some jobs use staggered hours (shifts, early/late starts).
  - Consumption trips bunch somewhat (lunch, evening) but with per-NPC jitter so demand curves are smooth.

## Product focus

- Develop **traffic system** deeply: roads, signals, signs, roundabouts, intersection behavior, congestion.
- City layer exists to **generate time-varying demand** on the network.

## Discarded / non-goals (for now)

- Full city sim, zoning, economy, multiplayer, 3D.
- Heavy city-builder where placing homes/jobs is the main loop (TBD — see input doc open questions).

## Open threads captured here

- Player builds only roads/signals vs places POIs?
- Real-time + pause vs tick-based sim?
- Victory conditions vs endless sandbox?
- Evening rush + consumption peaks: doc now, ship later?

## Changelog

| Date | Note |
|------|------|
| 2026-11-19 | Initial capture from design chat; promoted to input doc same day |