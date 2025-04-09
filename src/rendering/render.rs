use egui::epaint::CircleShape;
use egui::{Color32, Grid, Pos2, Shape, Stroke, Ui, Vec2};

use super::camera::Camera;
use crate::agent::agent::Agent;
use crate::environment::crop::Crop;
use crate::environment::obstacle::Obstacle;
use crate::path::visibility_graph::VisibilityGraph;
use crate::utilities::{pos2::ExtendedPos2, vec2::ExtendedVec2};



// region: SCENE

pub fn render_coordinate_system(ui: &mut Ui, camera: &Camera) {
    let painter = ui.painter();
    let width = camera.scene_to_screen_val(0.05);
    let stroke_x = Stroke::new(width, Color32::from_rgb(255, 0, 0));
    let stroke_y = Stroke::new(width, Color32::from_rgb(0, 255, 0));
    
    let zero = camera.scene_to_screen_pos(Pos2::ZERO);
    let xp = camera.scene_to_screen_pos(Pos2::new(1.0,0.0));
    let yp = camera.scene_to_screen_pos(Pos2::new(0.0,1.0));
    // Draw X axis
    let points_x = [zero, xp];
    painter.line_segment(points_x, stroke_x);
    
    // Draw Y axis
    let points_y = [zero, yp];
    painter.line_segment(points_y, stroke_y);
}

pub fn render_grid(ui: &mut Ui, camera: &Camera) {
    let min = -100;
    let max = 100;
    let painter = ui.painter();
    let color = Color32::from_rgba_unmultiplied(255, 255, 255, 10);
    let stroke = Stroke::new(0.05, color);
    for x in min..=max {
        let p1 = camera.scene_to_screen_pos(Pos2::new(x as f32,min as f32));
        let p2 = camera.scene_to_screen_pos(Pos2::new(x as f32,max as f32));
        let points = [p1, p2];
        painter.line_segment(points, stroke);
    }
    for y in min..=max {
        let p1 = camera.scene_to_screen_pos(Pos2::new(min as f32,y as f32));
        let p2 = camera.scene_to_screen_pos(Pos2::new(max as f32,y as f32));
        let points = [p1, p2];
        painter.line_segment(points, stroke);
    }
}

pub fn render_obstacles(ui: &mut Ui, camera: &Camera, obstacles: &Vec<Obstacle>) {
    let painter = ui.painter();
    for obs in obstacles {
        let points: Vec<Pos2> = obs.points
            .iter()
            .map(|pos| camera.scene_to_screen_pos(*pos))
            .collect();
        painter.add(Shape::convex_polygon(
            points,
            Color32::from_rgb(0, 0, 0),
            Stroke::NONE,
        ));
    }
}

pub fn render_crops(ui: &mut Ui, camera: &Camera, crops: &Vec<Crop>) {
    let painter = ui.painter();
    for crop in crops {
        let center = camera.scene_to_screen_pos(crop.position);
        let radius = camera.scene_to_screen_val(0.1);
        painter.add(CircleShape {
            center,
            radius,
            fill: Color32::GREEN,
            stroke: Stroke::default(),
        });
    }
}

pub fn render_agents(ui: &mut Ui, camera: &Camera, agents: &Vec<Agent>) {
    let painter = ui.painter();
    for agent in agents {
        let center = camera.scene_to_screen_pos(agent.position);
        let radius = camera.scene_to_screen_val(0.15);
        painter.add(CircleShape {
            center,
            radius,
            fill: Color32::RED,
            stroke: Stroke::default(),
        });
        let start = center;
        let end = center + (agent.direction*Vec2::new(1.0,-1.0)) * camera.scene_to_screen_val(0.5);
        let stroke = Stroke {
            width: camera.scene_to_screen_val(0.02),
            color: Color32::RED,
        };
        painter.line(vec![start.into(), end.into()], stroke);

        painter.add(CircleShape {
            center: end,
            radius: radius*0.5,
            fill: Color32::RED,
            stroke: Stroke::default(),
        });
        let stroke1 = Stroke {
            width: camera.scene_to_screen_val(0.02),
            color: Color32::MAGENTA,
        };
        if agent.path.is_some() {
            let mut path = vec![agent.position];
            path.extend(agent.path.as_ref().unwrap());
            if path.len() > 1 {
                for i in 0..path.len()-1 {
                    let start = camera.scene_to_screen_pos(path[i]);
                    let end = camera.scene_to_screen_pos(path[i+1]);
                    painter.line(vec![start.into(), end.into()], stroke1);
                }
            }
        }
    }
}

pub fn render_visibility_graph(ui: &mut Ui, camera: &Camera, visibility_graph: &VisibilityGraph) {
    // Draw edges
    let line_width = camera.scene_to_screen_val(0.05);
    for edge in visibility_graph.graph.edge_references() {
        let (a, b) = (petgraph::visit::EdgeRef::source(&edge), petgraph::visit::EdgeRef::target(&edge));
        let start = camera.scene_to_screen_pos(visibility_graph.graph[a]);
        let end = camera.scene_to_screen_pos(visibility_graph.graph[b]);
        ui.painter().line_segment(
            [start, end],
            (line_width, Color32::from_rgb(100, 100, 255)),
        );
    }

    // Draw nodes on top
    let node_radius = camera.scene_to_screen_val(0.01);
    for node in visibility_graph.graph.node_indices() {
        let pos = camera.scene_to_screen_pos(visibility_graph.graph[node]);
        ui.painter().circle_filled(pos, node_radius, Color32::RED);
    }
}


// endregion

// region: UI

pub fn ui_render_agents(ui: &mut Ui, agents: &Vec<Agent>) {
    Grid::new("agents")
    .striped(true)
    .show(ui, |ui| {
        ui.label("Agent Id");
        ui.label("Position");
        ui.label("Direction");
        ui.end_row();

        for agent in agents {
            ui.label(agent.id.to_string());
            ui.label(agent.position.fmt(2));
            ui.label(agent.direction.fmt(2));
            ui.end_row();
        }
    });
}

pub fn ui_render_agents_path(ui: &mut Ui, agents: &Vec<Agent>) {
    for agent in agents {
        ui.label(format!("{} {} {}", agent.id.to_string(), agent.position.fmt(2), agent.direction.fmt(2)));
        let mut path_str = String::new();
        if agent.path.is_some() {
            let path: &Vec<Pos2> = agent.path.as_ref().unwrap();
            for p in path {
                path_str += p.fmt(2).as_str();
            }
        } else {
            path_str = "None".to_string();
        }
        ui.label(format!("Path: {}", path_str));
    }
}

// endregion
