use egui::epaint::CircleShape;
use egui::{Color32, Grid, Pos2, RichText, Shape, Stroke, Ui, Vec2};

use super::camera::Camera;
use crate::agent::agent::Agent;
use crate::agent::battery::Battery;
use crate::environment::crop::Crop;
use crate::environment::field_config::VariantFieldConfig;
use crate::environment::obstacle::Obstacle;
use crate::environment::spawn_area::SpawnArea;
use crate::environment::station::Station;
use crate::path::visibility_graph::VisibilityGraph;
use crate::task::task::Task;
use crate::task::task_manager::TaskManager;
use crate::utilities::vec2::{ExtendedVec2, Vec2Rotate};
use crate::utilities::pos2::ExtendedPos2;



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
        
        if let Some(task) = &agent.current_task {
            let mut path = vec![agent.position];
            path.extend(task.get_path());
        
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
            (line_width, Color32::from_rgba_unmultiplied(100, 100, 255, 50)),
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

pub fn render_variant_field_configs(ui: &mut Ui, camera: &Camera, configs: &Vec<VariantFieldConfig>) {
    let painter = ui.painter();
    for config_variant in configs {
        match config_variant {
            VariantFieldConfig::Line(config) => {
                for i in 0..config.n_lines {
                    let p1 = config.left_top_pos + Vec2::new( i as f32*config.line_spacing, 0.0).rotate_degrees(config.angle);
                    let p2 = p1 + Vec2::new(0.0, config.length).rotate_degrees(config.angle);
                    let p1 = camera.scene_to_screen_pos(p1);
                    let p2 = camera.scene_to_screen_pos(p2);
                    let stroke = Stroke {
                        width: camera.scene_to_screen_val(0.02),
                        color: config.color,
                    };
                    painter.line(vec![p1, p2], stroke);
                }
            },
            VariantFieldConfig::Point(config) => {
                for i in 0..config.n_lines {
                    for j in 0..config.n_points_per_line {
                        let pos = config.left_top_pos + Vec2::new(config.line_spacing*i as f32, config.point_spacing*j as f32).rotate_degrees(config.angle);
                        let center = camera.scene_to_screen_pos(pos);
                        let radius = camera.scene_to_screen_val(0.10);
                        painter.add(CircleShape {
                            center,
                            radius,
                            fill: config.color,
                            stroke: Stroke::default(),
                        });
                    }
                }
            }
        }
    }
}

pub fn render_tasks_on_field(ui: &mut Ui, camera: &Camera, tasks: &Vec<Task>) {
    let painter = ui.painter();
    let color = Color32::LIGHT_BLUE;
    for task in tasks {
        let path = task.get_path().clone();
        let screen_path: Vec<Pos2> = path.iter().map(|pos| {
            camera.scene_to_screen_pos(*pos)
        }).collect();
        if screen_path.len() == 1 {
            painter.add(CircleShape {
                center: path[0],
                radius: camera.scene_to_screen_val(0.1),
                fill: color,
                stroke: Stroke::default(),
            });
        } else {
            path.windows(2).for_each(|window| {
                if let [p1, p2] = window {
                    painter.line(
                        vec![*p1, *p2], 
                        Stroke::new(camera.scene_to_screen_val(0.05), color),
                    );
                }
            });
        }
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
        ui.label("State");
        ui.label("Battery");
        ui.label("Current task");
        ui.label("Work Schedule");
        ui.end_row();
        
        for agent in agents {
            ui.label(RichText::new("⏺").color(agent.color)); //⏹⏺
            ui.label(agent.id.to_string());
            ui.label(agent.position.fmt(2));
            ui.label(agent.direction.fmt(2));
            ui.label(format!("{:?}",agent.state));
            ui.label(format!("{:.2}%",agent.battery.get_soc()));
            match &agent.current_task {
                Some(task) => {
                    ui.label(format!("{:?}", task.get_intent()));
                },
                None => { ui.label("False"); }
            }
            ui.label(agent.work_schedule.len().to_string());
            ui.end_row();
        }
    });
}

pub fn ui_render_agents_path(ui: &mut Ui, agents: &Vec<Agent>) {
    for agent in agents {
        ui.horizontal(|ui| {
            ui.label(RichText::new("⏺").color(agent.color));
            ui.label(format!(" {} {} {} {:?} {:.2}%", agent.id, agent.position.fmt(2), agent.direction.fmt(2), agent.state, agent.battery.get_soc()));
        });

        let mut path_str = String::new();
        if let Some(task) = &agent.current_task {
            for p in task.get_path() {
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
        ui.label("N slots");
        ui.label("Slots");
        ui.label("Queue");
        ui.end_row();

        for station in stations {
            ui.label(RichText::new("⏺").color(station.color)); //⏹⏺
            ui.label(station.id.to_string());
            ui.label(station.position.fmt(2));
            ui.label(station.n_slots.to_string());
            ui.label(station.slots.len().to_string());
            ui.label(station.queue.len().to_string());
            ui.end_row();
        }
    });
}

pub fn ui_render_mouse_screen_scene_pos(ui: &mut Ui, camera: &Camera) {
    let (mouse_pos, scene_pos) = match camera.mouse_position {
        Some(pos) => {
            let scene_pos = camera.screen_to_scene_pos(pos);
            (Some(pos), Some(scene_pos))
        },
        None => {
            (None, None)
        },
    };
    ui.label(format!("Mouse pos: {}", mouse_pos.map_or("None".to_string(), |p| p.fmt(2))));
    ui.label(format!("Scene pos: {}", scene_pos.map_or("None".to_string(), |p| p.fmt(2))));
}

pub fn ui_render_task_manager(ui: &mut Ui, task_manager: &TaskManager) {
    fn make_grid_from(ui: &mut Ui, label: String, vec: Vec<Task>) {
        ui.collapsing(format!("{} ({})", label, vec.len()), |ui| {
            Grid::new(label.to_string())
                .striped(true)
                .show(ui, |ui| {
                    // Header row for the grid
                    ui.label("Id");
                    ui.label("Type");
                    ui.label("Path");
                    ui.label("Velocity");
                    ui.label("Duration");
                    ui.label("Field id");
                    ui.label("Line id");
                    ui.label("Power w");
                    ui.end_row();

                    struct TaskInfo {
                        id: u32, task_type: String, path: Vec<String>, vel: String, dur: String, fid: u32, lid: u32, power: f32
                    }

                    fn display_task_info(ui: &mut Ui, task_info: TaskInfo) {
                        ui.label(task_info.id.to_string());
                        ui.label(task_info.task_type);
                        ui.label(task_info.path.join(", "));
                        ui.label(task_info.vel);
                        ui.label(task_info.dur);
                        ui.label(task_info.fid.to_string());
                        ui.label(task_info.lid.to_string());
                        ui.label(task_info.power.to_string());
                        ui.end_row();
                    }

                    for task in vec {
                        match task {
                            Task::Stationary { id, pos, duration, field_id, line_id, power_w ,..} => {
                                let task_type = "Stationary";
                                let path = vec![pos.fmt(2)];
                                let vel = "-".to_string();
                                let dur = duration.to_string();
                                let fid = field_id;
                                let lid = line_id;
                                let power = power_w;
                                
                                display_task_info(ui, TaskInfo { id, task_type: task_type.to_string(), path, vel, dur, fid, lid, power } );
                            }
                            Task::Moving { id, path, velocity, field_id, line_id, power_w ,..} => {
                                let task_type = "Moving";
                                let path: Vec<String> = path.iter()
                                    .map(|pos| pos.fmt(2))
                                    .collect();
                                let vel = velocity.to_string();
                                let dur = "-".to_string();
                                let fid = field_id;
                                let lid = line_id;
                                let power = power_w;
                                
                                display_task_info(ui, TaskInfo { id, task_type: task_type.to_string(), path, vel, dur, fid, lid, power } );
                            }
                            _ => {}
                        }
                    }
                });
        });
    }
    
    ui.label("Task manager");
    make_grid_from(ui, "Work List".to_string(), <std::collections::VecDeque<Task> as Clone>::clone(&task_manager.work_list).into());
    make_grid_from(ui, "Assigned List".to_string(), task_manager.assigned_tasks.clone());
    make_grid_from(ui, "Completed List".to_string(), task_manager.completed_tasks.clone());  
}

// endregion
