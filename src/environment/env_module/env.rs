use egui::Vec2;

use crate::{
    agent_module::agent::Agent,
    environment::{
        datetime::{DateTimeConfig, DateTimeManager}, env_module::env_config::EnvConfig, field_config::FieldConfig, obstacle::Obstacle, spawn_area_module::spawn_area::SpawnArea, station_module::station::Station
    },
    path_finding_module::visibility_graph::VisibilityGraph,
    task_module::task_manager::TaskManager,
    units::duration::Duration,
    utilities::{
        pos2::random_pos2_in_rect, utils::{generate_colors, load_json}, vec2::random_vec2
    }
};

#[derive(Debug, Clone)]
pub struct Env {
    pub step_count: u32,
    pub n_agents: u32,
    pub agent_path: String,
    pub agents: Vec<Agent>,
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
                Agent::from_config(
                    load_json(config.agent_path.clone()).expect("Can't deserialize to AgentConfig"),
                    i,
                    random_pos2_in_rect(egui::Rect { min: spawn_area.left_top_pos, max: spawn_area.left_top_pos+Vec2::new(spawn_area.width.to_base_unit(), spawn_area.height.to_base_unit()) }, spawn_area.angle),
                    random_vec2(),
                    agent_colors[i as usize]
                )
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

        let task_manager = TaskManager::from_field_config(field_config.clone());
        Self {
            step_count: 0,
            n_agents,
            agent_path: config.agent_path,
            agents,
            field_config,
            stations,
            spawn_area,
            obstacles,
            visibility_graph,
            datetime_config: config.datetime_config,
            date_time_manager,
            task_manager,
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
            self.agent_path.clone(),
            self.datetime_config.clone(),
            self.field_config.configs.clone(),
            station_configs,
            spawn_area_config
        )
    }
    
    pub fn reset(&mut self) {
        self.agents.clear();
        let agent_colors = generate_colors(self.n_agents as usize, 0.1);
        for i in 0..self.n_agents {
            self.agents.push(
                Agent::from_config(
                    load_json(self.agent_path.clone()).expect("Can't deserialize to AgentConfig"),
                    i,
                    random_pos2_in_rect(egui::Rect { min: self.spawn_area.left_top_pos, max: self.spawn_area.left_top_pos+Vec2::new(self.spawn_area.width.to_base_unit(), self.spawn_area.height.to_base_unit()) }, self.spawn_area.angle),
                    random_vec2(),
                    agent_colors[i as usize]
                )
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
        let simulation_step = Duration::seconds(1.0);
        self.step_count += 1;
        self.date_time_manager.advance_time(1);
        self.task_manager.update_waiting_list(simulation_step);
        for agent in &mut self.agents {
            agent.update(simulation_step, &self.date_time_manager);
        }
    }
}


