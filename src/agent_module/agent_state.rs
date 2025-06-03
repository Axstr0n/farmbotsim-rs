use std::collections::HashMap;

use crate::{
    agent_module::{
        agent::Agent,
        battery::Battery,
    },
    environment::datetime::DateTimeManager,
    task_module::task::Task,
    units::{
        duration::Duration,
        power::Power,
    },
    cfg::{
        POWER_CONSUMPTION_WAIT, POWER_CONSUMPTION_TRAVEL, MAX_VELOCITY,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub enum AgentState {
    Wait,
    Travel,
    Work,
    Charging,
    Discharged,
}

impl AgentState {
    pub fn on_enter(&mut self, agent: &mut Agent) {
        match self {
            AgentState::Wait => { },
            AgentState::Travel => { },
            AgentState::Work => { },
            AgentState::Charging => {
                agent.battery.start_index = HashMap::from([
                    ("jan".to_string(), 1),
                    ("jun".to_string(), 1),
                ]);
            },
            AgentState::Discharged => { },
        }
    }

    pub fn update(&mut self, agent: &mut Agent, date_time_manager: &DateTimeManager) -> Option<AgentState> {

        fn check_battery(agent: &Agent) -> Option<AgentState> {
            if agent.battery.get_soc() <= 0.0 {
                return Some(AgentState::Discharged);
            }
            None
        }
        fn calculate_power_travel(agent: &Agent) -> Power {
            POWER_CONSUMPTION_TRAVEL * (agent.velocity_lin / MAX_VELOCITY)
        }
        fn calculate_power_work(agent: &Agent, task: &Task) -> Power {
            match task {
                Task::Stationary { power, .. } => *power,
                Task::Moving { power, .. } => *power + calculate_power_travel(agent),
                _ => Power::watts(0.0)
            }
        }

        match self {
            AgentState::Wait => {
                // discharge battery
                agent.battery.discharge(POWER_CONSUMPTION_WAIT, Duration::seconds(1.0));
                // check battery
                if let Some(discharge) = check_battery(agent) { return Some(discharge); }
                // transitions
                if let Some(task) = &agent.current_task {
                    if task.is_wait() { None }
                    else if task.is_travel() { Some(AgentState::Travel) }
                    else { None }
                }
                else { None }
            },
            AgentState::Travel => {
                // discharge battery
                let power = calculate_power_travel(agent);
                agent.battery.discharge(power, Duration::seconds(1.0));
                // check battery
                if let Some(discharge) = check_battery(agent) { return Some(discharge); }
                // transitions
                if let Some(task) = &agent.current_task {
                    if task.is_work() { Some(AgentState::Work) }
                    else if task.is_wait() && task.is_charge_intent() { Some(AgentState::Charging) }
                    else if task.is_wait() { Some(AgentState::Wait) }
                    else { None }
                }
                else { Some(AgentState::Wait) }
            },
            AgentState::Work => {
                // discharge battery
                let power = match &agent.current_task {
                    Some(task) => {
                        calculate_power_work(agent, task)
                    },
                    None => {
                        Power::watts(0.0) // when task is complete and removed but state is still Work instead of other
                    }
                };
                agent.battery.discharge(power, Duration::seconds(1.0));
                // check battery
                if let Some(discharge) = check_battery(agent) { return Some(discharge); }
                // transitions
                if let Some(task) = &agent.current_task {
                    if !task.is_work() { Some(AgentState::Travel) }
                    else { None }
                }
                else { Some(AgentState::Wait) }
            },
            AgentState::Charging => {
                // charge battery
                agent.battery.charge(Duration::seconds(1.0), date_time_manager.get_month());
                // transitions
                if let Some(task) = &agent.current_task {
                    if !task.is_wait() && !task.is_charge_intent() { Some(AgentState::Travel) }
                    else { None }
                }
                else { None }
            },
            AgentState::Discharged => {
                None
            },
        }
    }

    pub fn on_exit(&mut self, _agent: &mut Agent) {
        match self {
            AgentState::Wait => { },
            AgentState::Travel => { },
            AgentState::Work => { },
            AgentState::Charging => { },
            AgentState::Discharged => { },
        }
    }
}
