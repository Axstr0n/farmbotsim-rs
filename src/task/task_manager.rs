use std::collections::VecDeque;

use egui::Vec2;

use crate::{agent::agent::Agent, environment::field_config::{FieldConfig, VariantFieldConfig}, path::visibility_graph::VisibilityGraph, utilities::vec2::Vec2Rotate};

use super::task::Task;



pub struct TaskManager {
    id_counter: u32,
    pub all_tasks: Vec<Task>,
    work_list: VecDeque<Task>,
    assigned_tasks: Vec<Task>,
    completed_tasks: Vec<Task>,
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
                        work_list.push(Task::moving(id_counter, path, 5.0, Some(field_id), Some(i)));
                        id_counter += 1;
                    }
                },
                VariantFieldConfig::Point(c) => {
                    for i in 0..c.n_lines {
                        for j in 0..c.n_points_per_line {
                            let pos = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, c.point_spacing*j as f32).rotate_degrees(c.angle);
                            work_list.push(Task::stationary(id_counter, pos, 40.0, Some(field_id), Some(i)));
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
        for agent in agents {
            if agent.work_schedule.is_empty() {
                self.assign_task_to_agent(agent);
            }
        }
    }
    fn assign_task_to_agent(&mut self, agent: &mut Agent) -> bool {
        let task_opt = self.work_list.pop_front();
        if let Some(task) = task_opt {
            let path = self.visibility_graph.find_path(agent.position, task.path[0]);
            if let Some(p) = path {
                agent.work_schedule.push_back(Task::moving(0, p, 4.0, None, None)); // Travel to task
                agent.work_schedule.push_back(task.clone());
                self.assigned_tasks.push(task);
                return true
            }
        }
        false
    }
}