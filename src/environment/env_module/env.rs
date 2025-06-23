use egui::Vec2;

use crate::{
    agent_module::agent::Agent,
    environment::{
        datetime::{DateTimeConfig, DateTimeManager}, env_module::env_config::EnvConfig, field_config::FieldConfig, obstacle::Obstacle, scene_config::SceneConfig, spawn_area_module::spawn_area::SpawnArea, station_module::station::Station
    },
    path_finding_module::visibility_graph::VisibilityGraph,
    task_module::task_manager::TaskManager,
    units::duration::Duration,
    utilities::{
        pos2::random_pos2_in_rect, utils::{generate_colors, load_json_or_panic}, vec2::random_vec2
    }
};

/// Represents the environment of the simulation including agents, field, stations, obstacles,
/// and management of time and tasks.
#[derive(Debug, Clone)]
pub struct Env {
    /// Number of simulation steps performed.
    pub step_count: u32,
    /// Total duration elapsed.
    pub duration: Duration,
    /// Number of agents in the environment.
    pub n_agents: u32,
    /// File path to the agent configuration.
    pub agent_path: String,
    /// Collection of agents in the environment.
    pub agents: Vec<Agent>,
    /// Configuration of the field layout.
    pub field_config: FieldConfig,
    /// List of stations in the environment.
    pub stations: Vec<Station>,
    /// Spawn area for agent placement.
    pub spawn_area: SpawnArea,
    /// Obstacles present in the env.
    pub obstacles: Vec<Obstacle>,
    /// Visibility graph used for pathfinding.
    pub visibility_graph: VisibilityGraph,
    /// Configuration of the datetime system.
    pub datetime_config: DateTimeConfig,
    /// Manages date and time.
    pub date_time_manager: DateTimeManager,
    /// Manages tasks assigned to agents.
    pub task_manager: TaskManager,
}

impl Env {
    /// Creates a new `Env` instance from a given `EnvConfig`.
    /// Panics if any JSON file can't be parsed or is not present.
    pub fn from_config(config: EnvConfig) -> Self {
        let scene_config: SceneConfig = load_json_or_panic(config.scene_config_path);
        let field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path);
        let spawn_area = SpawnArea::from_config(scene_config.spawn_area_config.clone());
        
        let n_agents = config.n_agents;
        let agent_colors = generate_colors(n_agents as usize, 0.1);
        let mut agents = Vec::new();
        for i in 0..n_agents {
            agents.push(
                Agent::from_config(
                    load_json_or_panic(config.agent_config_path.clone()),
                    i,
                    random_pos2_in_rect(egui::Rect { min: spawn_area.left_top_pos, max: spawn_area.left_top_pos+Vec2::new(spawn_area.width.to_base_unit(), spawn_area.height.to_base_unit()) }, spawn_area.angle),
                    random_vec2(),
                    agent_colors[i as usize]
                )
            )
        }
        
        let station_colors = generate_colors(scene_config.station_configs.len(), 0.0);
        let mut stations = Vec::new();
        for (i, station_config) in scene_config.station_configs.iter().enumerate() {
            stations.push(Station::from_config(i as u32, station_colors[i], station_config.clone()))
        }
        let obstacles = field_config.get_obstacles();
        let visibility_graph = VisibilityGraph::new(&field_config.get_graph_points(), obstacles.clone());

        let date_time_manager = DateTimeManager::from_config(config.datetime_config.clone());

        let task_manager = TaskManager::from_config(config.task_manager_config, field_config.clone());
        Self {
            step_count: 0,
            duration: Duration::ZERO,
            n_agents,
            agent_path: config.agent_config_path,
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

    /// Resets the environment to its initial state.
    pub fn reset(&mut self) {
        self.agents.clear();
        let agent_colors = generate_colors(self.n_agents as usize, 0.1);
        for i in 0..self.n_agents {
            self.agents.push(
                Agent::from_config(
                    load_json_or_panic(self.agent_path.clone()),
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

    /// Advances the environment by one step.
    pub fn step(&mut self) {
        let simulation_step = Duration::seconds(1.0);
        self.duration = self.duration + simulation_step;
        self.step_count += 1;
        self.date_time_manager.advance_time(simulation_step.to_base_unit() as i64);
        self.task_manager.update_waiting_list(simulation_step);
        for agent in &mut self.agents {
            agent.update(simulation_step, &self.date_time_manager);
        }
    }
}
