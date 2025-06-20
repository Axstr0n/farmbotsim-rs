use std::{collections::{HashMap, VecDeque}};

use egui::Pos2;

use crate::{
    agent_module::{
        agent::Agent,
        agent_state::AgentState,
        battery::Battery
    }, cfg::{MAX_VELOCITY, MAX_VELOCITY_BETWEEN_POINTS}, environment::{
        farm_entity_module::{
            farm_entity::FarmEntity,
            farm_entity_action::FarmEntityActionInstance,
            farm_stages::FarmStages
        },
        field_config::FieldConfig,
        station_module::station::{Station, StationPosType}
    }, logger::log_error_and_panic, movement_module::pose::path_to_poses, path_finding_module::{
        path_finding::PathFinding,
        visibility_graph::VisibilityGraph,
    }, task_module::task_manager_config::TaskManagerConfig, units::duration::Duration};
use super::task::{Intent, Task};


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChooseStationStrat {
    First,
    Closest,
    ClosestMinQueue,
}
impl ChooseStationStrat {
    pub fn variants() -> Vec<ChooseStationStrat> {
        vec![ChooseStationStrat::First, ChooseStationStrat::Closest, ChooseStationStrat::ClosestMinQueue]
    }
}
impl std::fmt::Display for ChooseStationStrat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::First => "First".to_string(),
            Self::Closest => "Closest".to_string(),
            Self::ClosestMinQueue => "ClosestMinQueue".to_string(),
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargingStrat {
    CriticalOnly,
    ThresholdWithLimit,
}
impl ChargingStrat {
    pub fn variants() -> Vec<ChargingStrat> {
        vec![ChargingStrat::CriticalOnly, ChargingStrat::ThresholdWithLimit]
    }
}
impl std::fmt::Display for ChargingStrat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::CriticalOnly => "CriticalOnly".to_string(),
            Self::ThresholdWithLimit => "ThresholdWithLimit".to_string(),
        };
        write!(f, "{str}")
    }
}

#[derive(Debug, Clone)]
pub struct TaskManager {
    id_counter: u32,
    field_config: FieldConfig,
    
    pub farm_entities: HashMap<u32, FarmEntity>,
    pub waiting: HashMap<u32, Duration>, // stores and decremend all waiting actions

    pub work_list: VecDeque<Task>,
    pub assigned_tasks: Vec<Task>,
    pub completed_tasks: Vec<Task>,
    visibility_graph: VisibilityGraph,

    pub charging_strat: ChargingStrat,
    pub choose_station_strat: ChooseStationStrat,
}

impl TaskManager {
    pub fn from_config(task_manager_config: TaskManagerConfig, field_config: FieldConfig) -> Self {
        let farm_entities = field_config.get_farm_entities();
        let (id_counter, work_list) = Self::get_initial_work_list(&farm_entities);
        let obstacles = field_config.get_obstacles();
        let visibility_graph = VisibilityGraph::new(&field_config.get_graph_points(), obstacles);
        Self {
            id_counter,
            field_config,
            farm_entities,
            waiting: HashMap::new(),
            work_list,
            assigned_tasks: vec![],
            completed_tasks: vec![],
            visibility_graph,
            charging_strat: task_manager_config.charging_strat,
            choose_station_strat: task_manager_config.choose_station_strat,
        }
    }

    pub fn to_config(&self) -> TaskManagerConfig {
        TaskManagerConfig { charging_strat: self.charging_strat.clone(), choose_station_strat: self.choose_station_strat.clone() }
    }

    pub fn reset(&mut self) {
        let farm_entities = self.field_config.get_farm_entities();
        let (id_counter, work_list) = Self::get_initial_work_list(&farm_entities);
        self.id_counter = id_counter;
        self.work_list = work_list;
        self.assigned_tasks.clear();
        self.completed_tasks.clear();
    }

    fn get_initial_work_list(farm_entities: &HashMap<u32, FarmEntity>) -> (u32, VecDeque<Task>) {
        let mut work_list = VecDeque::new();
        let mut task_id_counter = 0;
        for entity in farm_entities.values() {
            let task = entity.stages()[0].to_task(task_id_counter);
            if let Some(task) = task {
                work_list.push_back(task);
                task_id_counter += 1;
            }
        }

        (task_id_counter, work_list)
    }

    fn on_work_task_completed(&mut self, task: Task) {
        if let Some(farm_entity_id) = task.get_farm_entity_id() {
            if let Some(entity) = self.farm_entities.get_mut(&farm_entity_id) {
                entity.increment_stage();
                if let Some(next_action_instance) = entity.get_next_action_instance() {
                    let next_task = next_action_instance.to_task(self.id_counter);
                    if let Some(next_task) = next_task {
                        self.id_counter += 1;
                        self.work_list.push_back(next_task);
                    } else if let FarmEntityActionInstance::Wait { id, duration , ..} = next_action_instance {
                        self.waiting.insert(id, duration);
                    }
                }
            }
        }
    }

    pub fn update_waiting_list(&mut self, duration_: Duration) {
        let mut finished_ids = Vec::new();
        for (&id, duration) in self.waiting.iter_mut() {
            *duration = *duration - duration_;
            if duration.value <= 0.0 {
                finished_ids.push(id);
            }
        }
        for id in finished_ids {
            self.waiting.remove(&id);
            self.add_new_task_for_id(id);
        }
    }
    
    fn add_new_task_for_id(&mut self, id: u32) {
        if let Some(entity) = self.farm_entities.get_mut(&id) {
            entity.increment_stage();
            let next_action_instance = entity.get_next_action_instance();
            if let Some(next_action_instance) = next_action_instance {
                let next_task = next_action_instance.to_task(self.id_counter);
                if let Some(next_task) = next_task {
                    self.id_counter += 1;
                    self.work_list.push_back(next_task);
                } else if let FarmEntityActionInstance::Wait { id, duration, .. } = next_action_instance {
                    self.waiting.insert(id, duration);
                }
            }
        }
    }

    pub fn assign_tasks(&mut self, agents: &mut Vec<Agent>, stations: &mut [Station]) {
        let mut agent_ids_updated: Vec<u32> = vec![];
        let mut station_ids_updated: Vec<u32> = vec![];
        for agent in &mut *agents {
            self.update_completed_tasks(agent);
            // Discharge agents
            if agent.state == AgentState::Discharged {
                agent_ids_updated.push(agent.id);
                // TO DO
                if agent.current_task.is_none() { continue; }
                if let Some(task) = &agent.current_task {
                    if *task.get_intent() == Intent::Work {
                        // Return work task
                        self.work_list.push_front(task.clone());
                        agent.current_task = None;
                    }
                    for ws_task in &agent.work_schedule.tasks {
                        if *ws_task.get_intent() == Intent::Work {
                            // Return work task
                            self.work_list.push_front(ws_task.clone());
                        }
                    }
                    agent.work_schedule.clear();
                    for station in stations.iter_mut() {
                        if station.slots.contains(&agent.id) || station.queue.contains(&agent.id) {
                            station.release_agent(agent.id);
                        }
                    }
                } else {
                    continue;
                }

                // target_id = agent.task.target_id
                // if "station" in target_id:
                //     self.stations[target_id].release_agent(agent)
                // if "crop" in target_id:
                //     row_id = f'row_{agent.task.target_id.split("_")[1]}'
                //     self.crop_field.rows_assign[row_id] = False
                //     agent.task.object.quit_work()
                //     self.crop_field.update_row_processing_status()
                // agent.task = None
            }
            // Charging agents that are full
            else if agent.state == AgentState::Charging && agent.battery.get_soc() >= 100.0 {
                agent_ids_updated.push(agent.id);
                for station in &mut *stations {
                    if station.slots.contains(&agent.id) {
                        station.release_agent(agent.id);
                        station_ids_updated.push(station.id);
                    }
                }
                if !self.assign_work_tasks_to_agent(agent) {
                    self.assign_idle_tasks_to_agent(agent);
                }
            }
        }

        agent_ids_updated.extend(self.update_stations_on_agent_release(station_ids_updated, stations, agents));

        for agent in &mut *agents {
            if agent_ids_updated.contains(&agent.id) { continue; }
            // Agents going to station
            if agent.current_task.as_ref().map(|task| {
                let intent = task.get_intent();
                intent == &Intent::Charge || intent == &Intent::Queue
            }).unwrap_or(false) || agent.work_schedule.has_charging() {
                agent_ids_updated.push(agent.id);
            }
        }

        agent_ids_updated = self.charging_strategy(&mut agent_ids_updated, agents, stations);

        for agent in &mut *agents {
            if agent_ids_updated.contains(&agent.id) { continue; }

            // Agents that need to work
            if agent.current_task.is_none() && agent.work_schedule.is_empty() {
                self.assign_work_tasks_to_agent(agent);
            }
        }
    }


    pub fn update_stations_on_agent_release(&mut self, station_ids_updated: Vec<u32>, stations: &mut [Station], agents: &mut [Agent]) -> Vec<u32> {
        let mut agent_ids_updated = vec![];

        for station_id in station_ids_updated {
            if let Some(station) = stations.iter_mut().find(|s| s.id == station_id) {

                let queue_snapshot: Vec<u32> = station.queue.iter().cloned().collect();
                
                let mut updated_agents_count = 0; // moved in queue count
                for (i, agent_id) in queue_snapshot.iter().enumerate() {
                    if let Some(agent) = agents.iter_mut().find(|a| a.id == *agent_id) {
                        let pos: Pos2;
                        let intent: Intent;
                        if station.slots.len() < station.n_slots as usize { // Move in slot
                            pos = station.position;
                            intent = Intent::Charge;
                            station.release_agent(agent.id);
                            station.slots.push(*agent_id);
                            updated_agents_count += 1;
                        } else { // Move in queue
                            pos = station.get_waiting_position(i-updated_agents_count);
                            intent = Intent::Queue;
                        }
                        if let Some(path) = self.visibility_graph.find_path(agent.pose.position, pos) {
                            let path = path_to_poses(path);
                            let travel_task = Task::travel(path, MAX_VELOCITY, intent.clone());
                            let wait_task = Task::wait_infinite(intent);
                            agent.work_schedule.clear();
                            agent.work_schedule.push_back(travel_task);
                            agent.work_schedule.push_back(wait_task);
                            agent.current_task = agent.work_schedule.pop_front();
                            agent_ids_updated.push(agent.id);
                        }

                    }
                }
            }
        }
        agent_ids_updated
    }

    pub fn assign_station_tasks_to_agent(&mut self, agent: &mut Agent, stations: &mut [Station]) {
        let mut tasks_to_return: Vec<Task> = vec![];
        if let Some(task) = &agent.current_task {
            if task.is_work() { tasks_to_return.push(task.clone()); }
        }
        for task in &agent.work_schedule.tasks {
            if task.is_work() { tasks_to_return.push(task.clone()); }
        }
        //self.work_list.extend(tasks_to_return.clone());
        for task in tasks_to_return.clone().into_iter().rev() {
            self.work_list.push_front(task);
        }
        self.assigned_tasks.retain(|task| {
            !tasks_to_return.iter().any(|other_task| {
                task.get_id() == other_task.get_id()
            })
        });

        agent.current_task = None;
        agent.work_schedule.clear();
        let tasks = self.get_station_tasks(agent, stations);
        self.assign_tasks_to_agent(agent, tasks);
        agent.current_task = agent.work_schedule.pop_front();
    }

    pub fn assign_work_tasks_to_agent(&mut self, agent: &mut Agent) -> bool {
        let tasks = self.get_work_tasks(agent);
        if tasks.is_empty() { return false }
        self.assign_tasks_to_agent(agent, tasks);
        if let Some(task) = &agent.current_task {
            if task.is_wait() {
                agent.current_task = agent.work_schedule.pop_front();
            }
        }
        if agent.current_task.is_none() {
            agent.current_task = agent.work_schedule.pop_front();
        }
        true
    }

    pub fn assign_idle_tasks_to_agent(&mut self, agent: &mut Agent) -> bool {
        let tasks = self.get_idle_tasks(agent);
        if tasks.is_empty() { return false }
        self.assign_tasks_to_agent(agent, tasks);
        if let Some(task) = &agent.current_task {
            if task.is_wait() {
                agent.current_task = agent.work_schedule.pop_front();
            }
        }
        if agent.current_task.is_none() {
            agent.current_task = agent.work_schedule.pop_front();
        }
        true
    }


    pub fn get_station_tasks(&mut self, agent: &Agent, stations: &mut [Station]) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];
        let station_index = self.choose_station_index(agent, stations);
        let station = &mut stations[station_index];
        let (pos, pos_type) = station.request_charge(agent.id);
        let path = self.visibility_graph.find_path(agent.pose.position, pos);
        if let Some(path) = path {
            let path = path_to_poses(path);
            let intent = match pos_type {
                StationPosType::ChargingSlot => Intent::Charge,
                StationPosType::QueueSlot => Intent::Queue,
            };
            let task = Task::wait_infinite(intent.clone());
            let travel_task = Task::travel(path, MAX_VELOCITY, intent);
            tasks.push(travel_task);
            tasks.push(task);

        } else {
            station.release_agent(agent.id); // path to station was not found
        }

        tasks
    }

    pub fn get_work_tasks(&mut self, agent: &Agent) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];

        if let Some(task) = self.work_list.pop_front() {
            let mut related_tasks: Vec<_> = self.work_list
                .iter()
                .filter_map(|other| {
                    match other {
                        Task::Stationary { field_id, line_id, .. } => {
                            if let Task::Stationary { field_id: field_id_, line_id: line_id_, .. } = task.clone() {
                                if *field_id == field_id_ && *line_id == line_id_ {
                                    return Some(other.clone());
                                }
                            }
                            None
                        },
                        Task::Moving { field_id, farm_entity_id, .. } => {
                            if let Task::Stationary { field_id: field_id_, farm_entity_id: farm_entity_id_, .. } = task.clone() {
                                if *field_id == field_id_ && *farm_entity_id == farm_entity_id_ {
                                    return Some(other.clone());
                                }
                            }
                            None
                        }
                        _ => None,
                    }
                })
                .collect();
            related_tasks.push(task.clone());
            let reference_pos = agent.pose.position;
            related_tasks.sort_by(|a, b| {
                let a_pos = match a {
                    Task::Stationary { pose, .. } => pose.position,
                    _ => Pos2::ZERO, // shouldn't happen
                };
                let b_pos = match b {
                    Task::Stationary { pose, .. } => pose.position,
                    _ => Pos2::ZERO, // shouldn't happen
                };
                
                let a_distance = (a_pos.x - reference_pos.x).powi(2) + (a_pos.y - reference_pos.y).powi(2);
                let b_distance = (b_pos.x - reference_pos.x).powi(2) + (b_pos.y - reference_pos.y).powi(2);
            
                a_distance.partial_cmp(&b_distance).unwrap_or(std::cmp::Ordering::Equal)
            });
            let target_pose = related_tasks[0].get_first_pose();
            if let Some(target_pose) = target_pose {

                let path = self.visibility_graph.find_path(agent.pose.position, target_pose.position);
                if let Some(path) = path {
                    for (i,task) in related_tasks.iter().enumerate() {
                        let (velocity, path_) = match i {
                            0 => (MAX_VELOCITY, path.clone()),
                            _ => {
                                if let Some(pose) = task.get_first_pose() {(MAX_VELOCITY_BETWEEN_POINTS,vec![pose.position])}
                                else {(MAX_VELOCITY_BETWEEN_POINTS,vec![])}
                            },
                        };
                        let path_ = path_to_poses(path_);
                        tasks.push(Task::travel(path_, velocity, Intent::Work)); // Travel to task
                        tasks.push(task.clone()); // Task
                    }
                    for task_ in tasks.clone() {
                        if task_.is_work() { self.assigned_tasks.push(task_); }
                    }
                    
                    // Remove related tasks from work_list
                    self.work_list.retain(|task| {
                        !tasks.clone().iter().any(|related| task == related)
                    });
                } else {
                    self.work_list.push_front(task); // Add task back if path to it is None
                }
            }
        }

        tasks
    }

    pub fn get_idle_tasks(&mut self, agent: &Agent) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];

        let path = self.visibility_graph.find_path(agent.pose.position, agent.spawn_position);
        if let Some(path) = path {
            let path = path_to_poses(path);
            let travel_task = Task::travel(path, MAX_VELOCITY, Intent::Idle);
            //let wait_task = Task::wait_infinite(Intent::Idle);
            tasks.extend([travel_task]);
        }

        tasks
    }

    fn assign_tasks_to_agent(&mut self, agent: &mut Agent, tasks: Vec<Task>) {
        if tasks.is_empty() { return }
        for task in tasks {
            agent.work_schedule.push_back(task);
        }
    }

    pub fn update_completed_tasks(&mut self, agent: &mut Agent) {
        if !agent.completed_task_ids.is_empty() {
            let mut completed_task: Option<Task> = None;
            self.assigned_tasks.retain(|task| {
                if let Some(id) = task.get_id() {
                    if agent.completed_task_ids.contains(id) {
                        self.completed_tasks.push(task.clone());
                        completed_task = Some(task.clone());
                        false // Remove task from assigned_tasks
                    } else {
                        true  // Keep task in assigned_tasks
                    }
                } else { // If the task is Travel (no ID), keep it in assigned_tasks
                    true 
                }
            });
            if let Some(task) = completed_task {
                self.on_work_task_completed(task);
            }
            agent.completed_task_ids.clear();
        }
    }


    fn charging_strategy(&mut self, agent_ids_updated: &mut Vec<u32>, agents: &mut[Agent], stations: &mut [Station]) -> Vec<u32> {
        let critical_battery_level = 45.0;
        let low_battery_threshold = 60.0;
        match self.charging_strat {
            ChargingStrat::CriticalOnly => {
                for agent in agents {
                    if agent_ids_updated.contains(&agent.id) { continue; }
                    // If below critical battery go to charging
                    if agent.battery.get_soc() < critical_battery_level {
                        self.assign_station_tasks_to_agent(agent, stations);
                        agent_ids_updated.push(agent.id);
                    }
                }
            },
            ChargingStrat::ThresholdWithLimit => {
                let mut n_of_all_charging_agents = 0;
                for station in stations.iter() {
                    n_of_all_charging_agents += station.slots.len()+station.queue.len();
                }
                let max_agents_charging = stations.len();
                for agent in agents {
                    if agent_ids_updated.contains(&agent.id) { continue; }
                    if agent.battery.get_soc() < critical_battery_level {
                        // If below critical battery go to charging
                        self.assign_station_tasks_to_agent(agent, stations);
                        agent_ids_updated.push(agent.id);
                        n_of_all_charging_agents += 1;
                    }
                    else if agent.battery.get_soc() < low_battery_threshold && n_of_all_charging_agents < max_agents_charging {
                        // If not maximum number of charging agents and battery below threshold go charging
                        self.assign_station_tasks_to_agent(agent, stations);
                        agent_ids_updated.push(agent.id);
                        n_of_all_charging_agents += 1;
                    }
                }
            }
        }
        agent_ids_updated.to_vec()
    }

    fn choose_station_index(&mut self, agent: &Agent, stations: &[Station]) -> usize {
        match self.choose_station_strat {
            ChooseStationStrat::First => {
                0
            },
            ChooseStationStrat::Closest => {
                let mut distances = vec![];
                for station in stations {
                    let maybe_path = self.visibility_graph.find_path(agent.pose.position, station.position);
                    if let Some(path) = maybe_path {
                        let distance = path.windows(2)
                            .map(|w| w[0].distance(w[1]))
                            .sum();
                        distances.push(distance);
                    } else {
                        distances.push(f32::INFINITY)
                    }
                }
                let closest_maybe = distances.iter()
                    .enumerate()
                    .min_by(|a, b| {
                        a.1.partial_cmp(b.1).unwrap_or_else(|| {
                            let msg = format!(
                                "Failed to compare distances at indices {} and {}. Values: {} and {}",
                                a.0, b.0, a.1, b.1
                            );
                            log_error_and_panic(&msg)
                        })
                    })
                    .map(|(index, _)| index);
                closest_maybe.unwrap_or(0)
            },
            ChooseStationStrat::ClosestMinQueue => {
                let mut distances = vec![];
                for station in stations {
                    let maybe_path = self.visibility_graph.find_path(agent.pose.position, station.position);
                    if let Some(path) = maybe_path {
                        let distance: f32 = path.windows(2)
                            .map(|w| w[0].distance(w[1]))
                            .sum();
                        distances.push(distance + 40.0*(station.slots.len()+station.queue.len()) as f32);
                    } else {
                        distances.push(f32::INFINITY)
                    }
                }
                let closest_maybe = distances.iter()
                    .enumerate()
                    .min_by(|a, b| {
                        a.1.partial_cmp(b.1).unwrap_or_else(|| {
                            let msg = format!(
                                "Failed to compare distances at indices {} and {}. Values: {} and {}",
                                a.0, b.0, a.1, b.1
                            );
                            log_error_and_panic(&msg)
                        })
                    })
                    .map(|(index, _)| index);
                closest_maybe.unwrap_or(0)
            }
        }
    }
}