use egui::epaint::CircleShape;
use egui::{Color32, Grid, Pos2, RichText, Shape, Stroke, Ui, Vec2};

use super::camera::Camera;
use crate::agent::agent::Agent;
use crate::environment::crop::Crop;
use crate::environment::obstacle::Obstacle;
use crate::environment::spawn_area::SpawnArea;
use crate::environment::station::Station;
use crate::path::visibility_graph::VisibilityGraph;
use crate::utilities::vec2::Vec2Rotate;
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
            fill: agent.color,
            stroke: Stroke::default(),
        });
        let start = center;
        let end = center + (agent.direction*Vec2::new(1.0,-1.0)) * camera.scene_to_screen_val(0.5);
        let stroke = Stroke {
            width: camera.scene_to_screen_val(0.02),
            color: agent.color,
        };
        painter.line(vec![start, end], stroke);

        painter.add(CircleShape {
            center: end,
            radius: radius*0.5,
            fill: agent.color,
            stroke: Stroke::default(),
        });
        let stroke1 = Stroke {
            width: camera.scene_to_screen_val(0.02),
            color: Color32::MAGENTA,
        };
        
        if let Some(agent_path) = &agent.path {
            let mut path = vec![agent.position];
            path.extend(agent_path);
        
            if path.len() > 1 {
                for i in 0..path.len() - 1 {
                    let start = camera.scene_to_screen_pos(path[i]);
                    let end = camera.scene_to_screen_pos(path[i + 1]);
                    painter.line(vec![start, end], stroke1);
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

pub fn render_stations(ui: &mut Ui, camera: &Camera, stations: &Vec<Station>) {
    let painter = ui.painter();
    for station in stations {
        let center = camera.scene_to_screen_pos(station.position);
        let radius = camera.scene_to_screen_val(0.25);
        painter.add(CircleShape {
            center,
            radius,
            fill: station.color,
            stroke: Stroke::default(),
        });
        let radius = camera.scene_to_screen_val(0.20);
        painter.add(CircleShape {
            center,
            radius,
            fill: Color32::BLACK,
            stroke: Stroke::default(),
        });
    }
}

pub fn render_spawn_area(ui: &mut Ui, camera: &Camera, spawn_area: &SpawnArea) {
    let painter = ui.painter();
    let ltp = spawn_area.left_top_pos;
    let rtp = ltp + (Vec2::X * spawn_area.length).rotate_degrees(spawn_area.angle);
    let rbp = rtp + (Vec2::Y * spawn_area.width).rotate_degrees(spawn_area.angle);
    let lbp = ltp + (Vec2::Y * spawn_area.width).rotate_degrees(spawn_area.angle);
    let points = [ltp, rtp, rbp, lbp];
    let points: Vec<Pos2> = points
        .iter()
        .map(|pos| camera.scene_to_screen_pos(*pos))
        .collect();

    painter.add(Shape::convex_polygon(
        points,
        Color32::from_rgba_unmultiplied(0, 0, 0, 100),
        Stroke::NONE,
    ));
}

pub fn render_drag_points(ui: &mut Ui, camera: &Camera, points: &Vec<Pos2>) {
    let drag_point_size = camera.scene_to_screen_val(0.08);
    let painter = ui.painter();
    for pt in points {
        painter.add(egui::epaint::CircleShape {
            center: *pt,
            radius: drag_point_size,
            fill: egui::Color32::RED,
            stroke: egui::Stroke::NONE,
        });
    }
}

// endregion

// region: UI

pub fn ui_render_agents(ui: &mut Ui, agents: &Vec<Agent>) {
    ui.label("Agents");
    Grid::new("agents")
    .striped(true)
    .show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position");
        ui.label("Direction");
        ui.end_row();
        
        for agent in agents {
            ui.label(RichText::new("⏺").color(agent.color)); //⏹⏺
            ui.label(agent.id.to_string());
            ui.label(agent.position.fmt(2));
            ui.label(agent.direction.fmt(2));
            ui.end_row();
        }
    });
}

pub fn ui_render_agents_path(ui: &mut Ui, agents: &Vec<Agent>) {
    for agent in agents {
        ui.horizontal(|ui| {
            ui.label(RichText::new("⏺").color(agent.color));
            ui.label(format!(" {} {} {}", agent.id, agent.position.fmt(2), agent.direction.fmt(2)));
        });

        let mut path_str = String::new();
        if let Some(path) = &agent.path {
            for p in path {
                path_str += p.fmt(2).as_str();
            }
        } else {
            path_str = "None".to_string();
        }
        ui.label(format!("Path: {}", path_str));
    }
}

pub fn ui_render_stations(ui: &mut Ui, stations: &Vec<Station>) {
    ui.label("Stations");
    Grid::new("stations")
    .striped(true)
    .show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position");
        ui.label("Queue");
        ui.end_row();

        for station in stations {
            ui.label(RichText::new("⏺").color(station.color)); //⏹⏺
            ui.label(station.id.to_string());
            ui.label(station.position.fmt(2));
            ui.label(station.queue.len().to_string());
            ui.end_row();
        }
    });
}

// endregion
