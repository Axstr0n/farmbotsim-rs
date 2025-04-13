use egui::Vec2;

use crate::{environment::field_config::{FieldConfig, VariantFieldConfig}, utilities::vec2::Vec2Rotate};

use super::task::Task;



pub struct TaskManager {
    id_counter: u32,
    all_tasks: Vec<Task>,
    work_list: Vec<Task>,
    assigned_tasks: Vec<Task>,
    completed_tasks: Vec<Task>,
}

impl TaskManager {
    pub fn from_field_config(field_config: FieldConfig) -> Self {
        let (id_counter, all_tasks, work_list) = Self::generate_tasks(field_config);
        Self {
            id_counter,
            all_tasks,
            work_list,
            assigned_tasks: vec![],
            completed_tasks: vec![],
        }
    }
    fn generate_tasks(config: FieldConfig) -> (u32, Vec<Task>, Vec<Task>) {
        let mut work_list: Vec<Task> = vec![];
        let mut id_counter = 0;

        for config_variant in config.configs {
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    let path = vec![c.left_top_pos, c.left_top_pos+Vec2::new(0.0, c.length).rotate_degrees(c.angle)];
                    work_list.push(Task::moving(id_counter, path, 5.0));
                    id_counter += 1;
                },
                VariantFieldConfig::Point(c) => {
                    // Get all point positions
                    let mut positions = vec![];
                    for i in 0..c.n_lines {
                        for j in 0..c.n_points_per_line {
                            let pos = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, c.point_spacing*j as f32).rotate_degrees(c.angle);
                            positions.push(pos);
                        }
                    }
                    // Add tasks
                    for pos in positions {
                        work_list.push(Task::stationary(id_counter, pos, 40.0));
                    }
                }
            }
        }
        let all_tasks = work_list.clone();

        (id_counter, all_tasks, work_list)
    }
}