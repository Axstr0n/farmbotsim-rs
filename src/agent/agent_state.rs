use super::agent::Agent;


#[derive(Clone, Debug, PartialEq)]
pub enum AgentState {
    Wait,
    Travel,
    Work,
    Charging,
    Discharged,
}

impl AgentState {
    pub fn on_enter(&mut self, _agent: &mut Agent) {
        match self {
            AgentState::Wait => { },
            AgentState::Travel => { },
            AgentState::Work => { },
            AgentState::Charging => { },
            AgentState::Discharged => { },
        }
    }

    pub fn update(&mut self, agent: &mut Agent) -> Option<AgentState> {
        match self {
            AgentState::Wait => {
                if let Some(task) = &agent.current_task {
                    if !task.get_path().is_empty() {
                        Some(AgentState::Travel)
                    } else { None }
                }
                else { None }
            },
            AgentState::Travel => {
                if let Some(task) = &agent.current_task {
                    if task.is_work() { Some(AgentState::Work) }
                    else { None }
                }
                else { Some(AgentState::Wait) }
            },
            AgentState::Work => {
                if let Some(task) = &agent.current_task {
                    if !task.is_work() { Some(AgentState::Travel) }
                    else { None }
                }
                else { Some(AgentState::Wait) }
            },
            AgentState::Charging => {
                None
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
