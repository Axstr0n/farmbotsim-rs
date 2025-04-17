use std::collections::VecDeque;

use egui::{Pos2, Vec2};

use crate::{agent::{agent::Agent, agent_state::AgentState}, environment::field_config::{FieldConfig, VariantFieldConfig}, path::visibility_graph::VisibilityGraph, utilities::vec2::Vec2Rotate};

use super::task::Task;



pub struct TaskManager {
    id_counter: u32,
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
                    for i in 0..c.n_lines {
                        let path = vec![c.left_top_pos+Vec2::new(i as f32*c.line_spacing, 0.0).rotate_degrees(c.angle), c.left_top_pos+Vec2::new(i as f32*c.line_spacing, c.length).rotate_degrees(c.angle)];
                        work_list.push(Task::moving(id_counter, path, 5.0, field_id, i, 100.0));
                        id_counter += 1;
                    }
                },
                VariantFieldConfig::Point(c) => {
                    for i in 0..c.n_lines {
                        for j in 0..c.n_points_per_line {
                            let pos = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, c.point_spacing*j as f32).rotate_degrees(c.angle);
                            work_list.push(Task::stationary(id_counter, pos, 400.0, field_id, i, 50.0));
                            id_counter += 1;
                        }
                    }
                }
            }
        }
        let all_tasks = work_list.clone();

        (id_counter, all_tasks, work_list.into())
    }

    pub fn assign_tasks(&mut self, agents: &mut Vec<Agent>) {
        for agent in &mut *agents {
            if agent.current_task.is_none() && agent.work_schedule.is_empty() {
                if let Some(task) = self.work_list.pop_front() {
                    self.assign_task_to_agent(agent, task);
                }
            }
            self.update_completed_tasks(agent);
            if agent.state == AgentState::Discharged && !agent.work_schedule.is_empty() {

            }
        }
    }
    pub fn assign_task_to_agent(&mut self, agent: &mut Agent, task: Task) -> bool {
        let target_pos = task.get_first_pos();
        let path = self.visibility_graph.find_path(agent.position, *target_pos);
        if let Some(p) = path {
            agent.work_schedule.push_back(Task::travel(p, 4.0)); // Travel to task
            agent.work_schedule.push_back(task.clone()); // First task
            self.assigned_tasks.push(task.clone());

            // Chain stationary tasks in same line together
            let mut related_tasks: Vec<_> = self.work_list
                .iter()
                .filter_map(|other| {
                    match other {
                        Task::Stationary { field_id, line_id, .. } => {
                            if let Task::Stationary { field_id: fid, line_id: lid, .. } = task {
                                if field_id == &fid && line_id == &lid {
                                    return Some(other.clone());
                                }
                            }
                            None
                        }
                        _ => None,
                    }
                })
                .collect();
            let reference_pos = task.get_first_pos();
            related_tasks.sort_by(|a, b| {
                let a_pos = match a {
                    Task::Stationary { pos, .. } => pos,
                    _ => &Pos2::ZERO, // shouldn't happen
                };
                let b_pos = match b {
                    Task::Stationary { pos, .. } => pos,
                    _ => &Pos2::ZERO, // shouldn't happen
                };
                
                let a_distance = (a_pos.x - reference_pos.x).powi(2) + (a_pos.y - reference_pos.y).powi(2);
                let b_distance = (b_pos.x - reference_pos.x).powi(2) + (b_pos.y - reference_pos.y).powi(2);
            
                a_distance.partial_cmp(&b_distance).unwrap_or(std::cmp::Ordering::Equal)
            });

            for task in &related_tasks {
                agent.work_schedule.push_back(Task::travel(vec![*task.get_first_pos()], 0.5)); // Travel to task
                agent.work_schedule.push_back(task.clone()); // Task
            }

            for task_ in related_tasks.clone() {
                self.assigned_tasks.push(task_);
            }
            
            // Remove related tasks from work_list
            self.work_list.retain(|task| {
                !related_tasks.iter().any(|related| task == related)
            });

            return true
        }
        self.work_list.push_front(task); // Add task back if path to it is None
        false
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