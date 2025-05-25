use egui::Vec2;

use crate::agent_module::agent::Agent;
use crate::agent_module::movement::RombaMovement;
use crate::task_module::task_manager::TaskManager;

use super::env_config::EnvConfig;
use super::field::Field;
use super::field_config::FieldConfig;
use super::obstacle::Obstacle;
use super::station::Station;
use super::spawn_area::SpawnArea;
use crate::path_finding_module::visibility_graph::VisibilityGraph;
use crate::utilities::datetime::{DateTimeConfig, DateTimeManager};

use crate::utilities::pos2::random_pos2_in_rect;
use crate::utilities::vec2::random_vec2;
use crate::utilities::utils::generate_colors;

#[derive(Debug, Clone)]
pub struct Env {
    pub step_count: u32,
    pub n_agents: u32,
    pub agents: Vec<Agent>,
    pub field: Field,
    pub field_config: FieldConfig,
    pub stations: Vec<Station>,
    pub spawn_area: SpawnArea,
    pub obstacles: Vec<Obstacle>,
    pub visibility_graph: VisibilityGraph,
    pub datetime_config: DateTimeConfig,
    pub date_time_manager: DateTimeManager,
    pub task_manager: TaskManager,
}

impl Env {
    pub fn from_config(config: EnvConfig) -> Self {
        let spawn_area = SpawnArea::from_config(config.spawn_area_config);
        
        let n_agents = config.n_agents;
        let agent_colors = generate_colors(n_agents as usize, 0.1);
        let mut agents = Vec::new();
        for i in 0..n_agents {
            agents.push(
                Agent::new(i,
                    random_pos2_in_rect(egui::Rect { min: spawn_area.left_top_pos, max: spawn_area.left_top_pos+Vec2::new(spawn_area.width, spawn_area.height) }, spawn_area.angle),
                    random_vec2(),
                    RombaMovement::default(),
                agent_colors[i as usize])
            )
        }
        
        let station_colors = generate_colors(config.station_configs.len(), 0.0);
        let mut stations = Vec::new();
        for (i, station_config) in config.station_configs.iter().enumerate() {
            stations.push(Station::from_config(i as u32, station_colors[i], station_config.clone()))
        }
        let field_config = FieldConfig::new(config.field_configs);
        let obstacles = field_config.get_obstacles();
        let visibility_graph = VisibilityGraph::new(&field_config.get_graph_points(), obstacles.clone());

        let date_time_manager = DateTimeManager::from_config(config.datetime_config.clone());

        Self {
            step_count: 0,
            n_agents,
            agents,
            field: Field::default(),
            field_config: field_config.clone(),
            stations,
            spawn_area,
            obstacles,
            visibility_graph,
            datetime_config: config.datetime_config,
            date_time_manager,
            task_manager: TaskManager::from_field_config(field_config)
        }
    }
    
    pub fn to_config(&self) -> EnvConfig {
        let mut station_configs = vec![];
        for station in &self.stations {
            station_configs.push(station.to_config());
        }
        let spawn_area_config = self.spawn_area.to_config();
        EnvConfig::new(
            self.n_agents,
            self.datetime_config.clone(),
            self.field_config.configs.clone(),
            station_configs,
            spawn_area_config
        )
    }
    
    pub fn reset(&mut self) {
        self.agents.clear();
        let colors = generate_colors(self.n_agents as usize, 0.1);
        for i in 0..self.n_agents {
            self.agents.push(
                Agent::new(i,
                    random_pos2_in_rect(egui::Rect { min: self.spawn_area.left_top_pos, max: self.spawn_area.left_top_pos+Vec2::new(self.spawn_area.width, self.spawn_area.height) }, self.spawn_area.angle),
                    random_vec2(),
                    RombaMovement::default(),
                    colors[i as usize])
            )
        }
        for station in &mut self.stations {
            station.reset();
        }
        self.date_time_manager.reset();
        self.task_manager.reset();
        self.step_count = 0;
    }
    pub fn step(&mut self) {
        self.step_count += 1;
        self.date_time_manager.advance_time(1);
        for agent in &mut self.agents {
            agent.update(1, &self.date_time_manager);
        }
    }
}


