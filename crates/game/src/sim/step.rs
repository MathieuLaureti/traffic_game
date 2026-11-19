use traffic_agents::{decide_next_trip, AgentState, TripPurpose};
use traffic_pathing::find_path;
use traffic_traffic::{ActiveVehicle, VehicleSim};
use super::SimulationState;

pub fn advance_simulation(state: &mut SimulationState, dt: f32) {
    let prev_minute = state.clock.minute_of_day;
    state.clock.advance(dt);
    if state.clock.minute_of_day < prev_minute {
        for agent in &mut state.agents {
            agent.commuted_to_work_today = false;
            agent.returned_home_today = false;
        }
    }

  for controller in &mut state.controllers {
        controller.tick(dt);
    }

    let minute = state.clock.minute_of_day;
    let agents_snapshot: Vec<_> = state.agents.iter().cloned().collect();
    for agent in agents_snapshot {
        if state
            .vehicles
            .iter()
            .any(|v| v.agent_id == agent.id)
        {
            continue;
        }
        if let Some(plan) = decide_next_trip(
            &agent,
            minute,
            &state.scenario,
            state.scenario.commute_buffer_minutes,
        ) {
            if let Some(path) = find_path(&state.graph, plan.from, plan.to) {
                let idx = state
                    .agents
                    .iter()
                    .position(|a| a.id == plan.agent_id)
                    .expect("agent");
                state.agents[idx].state = AgentState::Commuting;
                state.vehicles.push(ActiveVehicle::new(
                    plan.agent_id,
                    path.nodes,
                    plan.purpose,
                    2.5,
                ));
            }
        }
    }

    let vehicle_snap: Vec<_> = state
        .vehicles
        .iter()
        .map(|v| (v.agent_id.clone(), v.position(), v.next_node()))
        .collect();

    let mut completed = Vec::new();
    for (i, vehicle) in state.vehicles.iter_mut().enumerate() {
        let current = vehicle.position();
        let next = vehicle.next_node().unwrap_or(current);
        let intersection = state.controllers.iter().find(|c| c.pos == next);
        let can_enter = if let Some(ctrl) = intersection {
            let on_ring = vehicle_snap
                .iter()
                .any(|(id, pos, _)| *pos == next && *id != vehicle.agent_id);
            ctrl.allows_entry(
                current,
                next,
                vehicle.wait_timer_secs,
                on_ring,
            )
        } else {
            true
        };

        if VehicleSim::advance(vehicle, dt, can_enter) {
            completed.push(i);
        }
    }

    let _ = vehicle_snap;

    completed.sort_unstable_by(|a, b| b.cmp(a));
    for i in completed {
        let vehicle = state.vehicles.remove(i);
        apply_arrival(state, &vehicle);
        state.metrics.trips_completed += 1;
    }

    let minutes_dt =
        dt * state.clock.time_scale * (traffic_sim_time::MINUTES_PER_DAY as f32 / state.clock.day_length_secs);
    for agent in &mut state.agents {
        if matches!(agent.state, AgentState::AtHome | AgentState::AtWork) {
            agent.needs.decay(minutes_dt);
        }
    }

    state.metrics.vehicles_active = state.vehicles.len() as u32;
    state.metrics.total_wait_secs += state
        .vehicles
        .iter()
        .map(|v| v.wait_timer_secs)
        .sum::<f32>()
        * dt;
}

fn apply_arrival(state: &mut SimulationState, vehicle: &ActiveVehicle) {
    let Some(agent) = state.agents.iter_mut().find(|a| a.id == vehicle.agent_id) else {
        return;
    };
    let dest = vehicle.route.last().copied().unwrap_or(agent.location);
    agent.location = dest;

    match vehicle.purpose {
        TripPurpose::CommuteToWork => {
            agent.state = AgentState::AtWork;
            agent.commuted_to_work_today = true;
        }
        TripPurpose::CommuteHome => {
            agent.state = AgentState::AtHome;
            agent.returned_home_today = true;
        }
        TripPurpose::Consume(need) => {
            agent.state = AgentState::AtHome;
            agent.needs.fill(need, 0.45);
            agent.location = agent.home;
        }
    }

}