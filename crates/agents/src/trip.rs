use crate::{planned_commute_depart, Agent, AgentState, ScheduleKind, TripPurpose};
use traffic_sim_time::MINUTES_PER_DAY;
use traffic_world::{NeedKind, ScenarioConfig, ServiceVariant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TripPlan {
    pub agent_id: String,
    pub from: traffic_world::GridPos,
    pub to: traffic_world::GridPos,
    pub purpose: TripPurpose,
}

pub fn decide_next_trip(
    agent: &Agent,
    minute_of_day: u32,
    scenario: &ScenarioConfig,
    commute_buffer: u32,
) -> Option<TripPlan> {
    match agent.state {
        AgentState::AtHome if agent.at_home_idling() => {
            if !agent.commuted_to_work_today
                && minute_window_hit(agent.morning_depart_minute, minute_of_day)
            {
                return Some(TripPlan {
                    agent_id: agent.id.clone(),
                    from: agent.home,
                    to: agent.workplace,
                    purpose: TripPurpose::CommuteToWork,
                });
            }
            if agent.commuted_to_work_today
                && !agent.returned_home_today
                && agent.needs.is_critical()
            {
                if let Some(need) = agent.needs.lowest_need() {
                    let variant = need_to_service(need);
                    if let Some(poi) = scenario.nearest_service(agent.home, variant) {
                        return Some(TripPlan {
                            agent_id: agent.id.clone(),
                            from: agent.home,
                            to: poi.access_road,
                            purpose: TripPurpose::Consume(need),
                        });
                    }
                }
            }
            if agent.commuted_to_work_today && !agent.returned_home_today {
                if let Some(need) = discretionary_need(agent, minute_of_day) {
                    let variant = need_to_service(need);
                    if let Some(poi) = scenario.nearest_service(agent.home, variant) {
                        return Some(TripPlan {
                            agent_id: agent.id.clone(),
                            from: agent.home,
                            to: poi.access_road,
                            purpose: TripPurpose::Consume(need),
                        });
                    }
                }
            }
        }
        AgentState::AtWork => {
            let end = agent.schedule.profile().work_end_minute;
            if minute_window_hit(end, minute_of_day) && !agent.returned_home_today {
                return Some(TripPlan {
                    agent_id: agent.id.clone(),
                    from: agent.workplace,
                    to: agent.home,
                    purpose: TripPurpose::CommuteHome,
                });
            }
        }
        AgentState::Consuming | AgentState::Commuting => {}
        AgentState::AtHome => {}
    }
    let _ = (commute_buffer, ScheduleKind::Standard95);
    None
}

fn minute_window_hit(target: u32, now: u32) -> bool {
    now >= target && now.saturating_sub(target) < 3
}

fn need_to_service(need: NeedKind) -> ServiceVariant {
    match need {
        NeedKind::Food => ServiceVariant::Food,
        NeedKind::Entertainment => ServiceVariant::Entertainment,
        NeedKind::Necessities => ServiceVariant::Necessities,
    }
}

fn discretionary_need(agent: &Agent, minute: u32) -> Option<NeedKind> {
    if agent.at_work() {
        return None;
    }
    let lunch = (11 * 60 + 30)..=(13 * 60 + 30);
    if lunch.contains(&minute) {
        return agent.needs.lowest_need();
    }
    let evening = (18 * 60)..(MINUTES_PER_DAY);
    if evening.contains(&minute) {
        if agent.needs.entertainment < NeedBars::COMFORT {
            return Some(NeedKind::Entertainment);
        }
    }
    agent.needs.lowest_need()
}

use crate::NeedBars;

pub fn spawn_agents(scenario: &ScenarioConfig) -> Vec<Agent> {
    let buffer = scenario.commute_buffer_minutes;
    scenario
        .agents
        .iter()
        .filter_map(|spawn| {
            let home = scenario.poi_by_id(&spawn.home_poi)?;
            let work = scenario.poi_by_id(&spawn.workplace_poi)?;
            let schedule = ScheduleKind::from_str_id(&spawn.schedule_profile);
            let morning_depart =
                planned_commute_depart(schedule, buffer, spawn.jitter_minutes);
            Some(Agent {
                id: spawn.id.clone(),
                home: home.access_road,
                workplace: work.access_road,
                schedule,
                jitter_minutes: spawn.jitter_minutes,
                state: AgentState::AtHome,
                needs: NeedBars::default(),
                location: home.access_road,
                morning_depart_minute: morning_depart,
                commuted_to_work_today: false,
                returned_home_today: false,
            })
        })
        .collect()
}