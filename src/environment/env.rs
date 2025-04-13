use egui::Pos2;

use crate::agent::agent::Agent;
use crate::agent::movement::RombaMovement;

use super::field::Field;
use super::field_config::FieldConfig;
use super::obstacle::Obstacle;
use super::station::Station;
use super::spawn_area::SpawnArea;
use crate::path::visibility_graph::VisibilityGraph;

use crate::utilities::pos2::random_pos2_in_rect;
use crate::utilities::vec2::random_vec2;
use crate::utilities::utils::generate_colors;

#[derive(Clone)]
pub struct Env {
    pub step_count: u32,
    n_agents: u32,
    pub agents: Vec<Agent>,
    pub field: Field,
    pub field_config: FieldConfig,
    pub stations: Vec<Station>,
    pub spawn_area: SpawnArea,
    pub obstacles: Vec<Obstacle>,
    pub visibility_graph: VisibilityGraph,
}

impl Env {
    pub fn new(n_agents: u32, field_config: Option<FieldConfig>) -> Self {
        let field_config = field_config.unwrap_or_default();
        let obstacles = field_config.get_obstacles();
        let visibility_graph = VisibilityGraph::new(&field_config.get_graph_points(), obstacles.clone());
        let colors = generate_colors(n_agents as usize, 0.1);
        let mut agents = Vec::new();
        for i in 0..n_agents {
            agents.push(
                Agent::new(i,
                random_pos2_in_rect(egui::Rect { min: Pos2::ZERO, max: Pos2::new(5.0,5.0) }),
                random_vec2(),
                RombaMovement::default(),
                visibility_graph.clone(),
                colors[i as usize])
            )
        }
        Self {
            step_count: 0,
            n_agents,
            agents,
            field: Field::default(),
            field_config,
            stations: vec![Station::default()],
            spawn_area: SpawnArea::default(),
            obstacles,
            visibility_graph,
        }
    }
    pub fn reset(&mut self) {
        self.agents.clear();
        let colors = generate_colors(self.n_agents as usize, 0.1);
        for i in 0..self.n_agents {
            self.agents.push(
                Agent::new(i,
                    random_pos2_in_rect(egui::Rect { min: Pos2::ZERO, max: Pos2::new(5.0,5.0) }),
                    random_vec2(),
                    RombaMovement::default(),
                    self.visibility_graph.clone(),
                    colors[i as usize])
            )
        }
        self.step_count = 0;
    }
    pub fn step(&mut self) {
        self.step_count += 1;
        for agent in &mut self.agents {
            agent.update(1);
        }
    }
}


