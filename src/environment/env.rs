use egui::{Pos2, Vec2};

use crate::agent::agent::Agent;
use crate::agent::movement::Movement;

use super::field::Field;
use super::field_config::FieldConfig;
use crate::path::visibility_graph::VisibilityGraph;

use crate::utilities::pos2::random_pos2_in_rect;
use crate::utilities::vec2::random_vec2;

#[derive(Clone)]
pub struct Env {
    pub step_count: u32,
    n_agents: u32,
    pub agents: Vec<Agent>,
    pub field: Field,
    pub visibility_graph: VisibilityGraph,
}

impl Default for Env {
    fn default() -> Self {
        let field = Field::from_config(None);
        let visibility_graph = VisibilityGraph::new(&field.get_graph_points(), field.obstacles.clone());
        Self {
            step_count: 0,
            n_agents: 1,
            agents: vec![
                Agent::new(0,
                    random_pos2_in_rect(egui::Rect { min: Pos2::ZERO, max: Pos2::new(5.0,5.0) }),
                    Vec2::Y,// random_vec2(),
                    Movement::default(),
                    visibility_graph.clone())
            ],
            field: field,
            visibility_graph: visibility_graph,
        }
    }
}

impl Env {
    pub fn new(n_agents: u32, field_config: Option<FieldConfig>) -> Self {
        let field = Field::from_config(field_config);
        let visibility_graph = VisibilityGraph::new(&field.get_graph_points(), field.obstacles.clone());
        let mut agents = Vec::new();
        for i in 0..n_agents {
            agents.push(
                Agent::new(i,
                random_pos2_in_rect(egui::Rect { min: Pos2::ZERO, max: Pos2::new(5.0,5.0) }),
                random_vec2(),
                Movement::default(),
                visibility_graph.clone())
            )
        }
        Self {
            step_count: 0,
            n_agents: n_agents,
            agents: agents,
            field: field,
            visibility_graph: visibility_graph,
        }
    }
    pub fn reset(&mut self) {
        self.agents.clear();
        for i in 0..self.n_agents {
            self.agents.push(
                Agent::new(i,
                    random_pos2_in_rect(egui::Rect { min: Pos2::ZERO, max: Pos2::new(5.0,5.0) }),
                    random_vec2(),
                    Movement::default(),
                    self.visibility_graph.clone())
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


