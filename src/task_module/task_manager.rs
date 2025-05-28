use std::collections::VecDeque;

use egui::{Pos2, Vec2};

use crate::{agent_module::{agent::Agent, agent_state::AgentState, battery::Battery}, cfg::{MAX_VELOCITY, MAX_VELOCITY_BETWEEN_POINTS}, environment::{field_config::{FieldConfig, VariantFieldConfig}, station::{Station, StationPosType}}, path_finding_module::visibility_graph::VisibilityGraph, units::{duration::Duration, linear_velocity::LinearVelocity, power::Power}, utilities::vec2::Vec2Rotate};
use crate::path_finding_module::path_finding::PathFinding;
use super::task::{Intent, MovingData, StationaryData, Task, WorkData};


#[derive(Debug, Clone)]
pub struct TaskManager {
    id_counter: u32,
    //pub fields: HashMap<u32, >
    pub all_tasks: Vec<Task>,
    pub work_list: VecDeque<Task>,
    pub assigned_tasks: Vec<Task>,
    pub completed_tasks: Vec<Task>,
    visibility_graph: VisibilityGraph,


}

impl TaskManager {
    pub fn from_field_config(field_config: FieldConfig) -> Self {
        let (id_counter, all_tasks, work_list) = Self::generate_tasks(&field_config);
        let obstacles = field_config.get_obstacles();
        let visibility_graph = VisibilityGraph::new(&field_config.get_graph_points(), obstacles);
        Self {
            id_counter,
            all_tasks,
            work_list,
            assigned_tasks: vec![],
            completed_tasks: vec![],
            visibility_graph,
        }
    }

    pub fn reset(&mut self) {
        self.work_list.clear();
        self.work_list = self.all_tasks.clone().into();
        self.assigned_tasks.clear();
        self.completed_tasks.clear();
    }

    fn generate_tasks(config: &FieldConfig) -> (u32, Vec<Task>, VecDeque<Task>) {
        let mut work_list: Vec<Task> = vec![];
        let mut id_counter = 0;

        for (n, config_variant) in config.configs.iter().enumerate() {
            let field_id = n as u32;
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    let ls_val = c.line_spacing.value;
                    for i in 0..c.n_lines {
                        let path = vec![c.left_top_pos+Vec2::new(i as f32*ls_val, 0.0).rotate(c.angle), c.left_top_pos+Vec2::new(i as f32*ls_val, c.length.to_base_unit()).rotate(c.angle)];
                        let work_data = WorkData::new(field_id, i, Power::watts(100.0));
                        let moving_data = MovingData::new(path, LinearVelocity::kilometers_per_hour(5.0), work_data);
                        work_list.push(Task::moving(id_counter, moving_data));
                        id_counter += 1;
                    }
                },
                VariantFieldConfig::Point(c) => {
                    for i in 0..c.n_lines {
                        for j in 0..c.n_points_per_line {
                            let pos = c.left_top_pos + Vec2::new(c.line_spacing.to_base_unit()*i as f32, c.point_spacing.to_base_unit()*j as f32).rotate(c.angle);
                            let work_data = WorkData::new(field_id, i, Power::watts(50.0));
                            let stationary_data = StationaryData::new(pos, Duration::seconds(400.0), work_data);
                            work_list.push(Task::stationary(id_counter, stationary_data));
                            id_counter += 1;
                        }
                    }
                }
            }
        }
        let all_tasks = work_list.clone();

        (id_counter, all_tasks, work_list.into())
    }

    pub fn assign_tasks(&mut self, agents: &mut Vec<Agent>, stations: &mut [Station]) {
        let mut agent_ids_updated: Vec<u32> = vec![];
        let mut station_ids_updated: Vec<u32> = vec![];
        for agent in &mut *agents {
            // Discharge agents
            if agent.state == AgentState::Discharged {
                agent_ids_updated.push(agent.id);
                // TO DO
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
                
            }
            
            
            // Agent that need to go to station
            else if agent.battery.get_soc() < 60.0 {
                self.assign_station_tasks_to_agent(agent, stations);
            }
            // Agents that need to work
            else if agent.current_task.is_none() && agent.work_schedule.is_empty() {
                self.assign_work_tasks_to_agent(agent);
            }
            self.update_completed_tasks(agent);
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
                        if let Some(path) = self.visibility_graph.find_path(agent.position, pos) {
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
        let station = &mut stations[0];
        let (pos, pos_type) = station.request_charge(agent.id);
        let path = self.visibility_graph.find_path(agent.position, pos);
        if let Some(path) = path {
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
                        Task::Stationary { data, .. } => {
                            if let Task::Stationary { data: data_, .. } = task.clone() {
                                if data.work_data.field_id == data_.work_data.field_id && data.work_data.line_id == data_.work_data.line_id {
                                    return Some(other.clone());
                                }
                            }
                            None
                        },
                        Task::Moving { data,.. } => {
                            if let Task::Stationary { data: data_, .. } = task.clone() {
                                if data.work_data.field_id == data_.work_data.field_id && data.work_data.line_id == data_.work_data.line_id {
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
            let reference_pos = agent.position;
            related_tasks.sort_by(|a, b| {
                let a_pos = match a {
                    Task::Stationary { data, .. } => data.pos,
                    _ => Pos2::ZERO, // shouldn't happen
                };
                let b_pos = match b {
                    Task::Stationary { data, .. } => data.pos,
                    _ => Pos2::ZERO, // shouldn't happen
                };
                
                let a_distance = (a_pos.x - reference_pos.x).powi(2) + (a_pos.y - reference_pos.y).powi(2);
                let b_distance = (b_pos.x - reference_pos.x).powi(2) + (b_pos.y - reference_pos.y).powi(2);
            
                a_distance.partial_cmp(&b_distance).unwrap_or(std::cmp::Ordering::Equal)
            });
            let target_pos = related_tasks[0].get_first_pos();
            if let Some(target_pos) = target_pos {

                let path = self.visibility_graph.find_path(agent.position, *target_pos);
                if let Some(path) = path {
                    for (i,task) in related_tasks.iter().enumerate() {
                        let (velocity, path_) = match i {
                            0 => (MAX_VELOCITY, path.clone()),
                            _ => {
                                if let Some(pos) = task.get_first_pos() {(MAX_VELOCITY_BETWEEN_POINTS,vec![*pos])}
                                else {(MAX_VELOCITY_BETWEEN_POINTS,vec![])}
                            },
                        };
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

        let path = self.visibility_graph.find_path(agent.position, agent.spawn_position);
        if let Some(path) = path {
            let travel_task = Task::travel(path, MAX_VELOCITY, Intent::Idle);
            let wait_task = Task::wait_infinite(Intent::Idle);
            tasks.extend([travel_task, wait_task]);
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
            self.assigned_tasks.retain(|task| {
                if let Some(id) = task.get_id() {
                    if agent.completed_task_ids.contains(id) {
                        self.completed_tasks.push(task.clone());
                        false // Remove task from assigned_tasks
                    } else {
                        true  // Keep task in assigned_tasks
                    }
                } else { // If the task is Travel (no ID), keep it in assigned_tasks
                    true 
                }
            });
            agent.completed_task_ids.clear();
        }
    }

}