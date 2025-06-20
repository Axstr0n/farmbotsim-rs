use egui::epaint::CircleShape;
use egui::{Align2, Color32, Grid, Pos2, RichText, Shape, Stroke, Ui, Vec2};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints, Text};

use crate::{
    agent_module::{
        agent::Agent,
        battery::Battery,
    },
    environment::{
        datetime::DateTimeManager,
        field_config::{FieldConfig, VariantFieldConfig},
        obstacle::Obstacle,
        spawn_area_module::spawn_area::SpawnArea,
        station_module::station::Station,
    },
    path_finding_module::visibility_graph::VisibilityGraph,
    rendering::camera::Camera,
    task_module::{
        task::Task,
        task_manager::TaskManager,
    },
    units::power::Power,
    utilities::{
        vec2::{ExtendedVec2, Vec2Rotate},
        pos2::ExtendedPos2,
    }
};


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

pub fn render_agents(ui: &mut Ui, camera: &Camera, agents: &Vec<Agent>) {
    let painter = ui.painter();
    for agent in agents {
        let center = camera.scene_to_screen_pos(agent.pose.position);
        let radius = camera.scene_to_screen_val(0.15);
        painter.add(CircleShape {
            center,
            radius,
            fill: agent.color,
            stroke: Stroke::default(),
        });
        let start = center;
        let end = center + (agent.pose.direction*Vec2::new(1.0,-1.0)) * camera.scene_to_screen_val(0.5);
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
            let mut path = vec![agent.pose.clone()];
            if let Some(path_) = task.get_path() {
                path.extend(path_);
            }
        
            if path.len() > 1 {
                for i in 0..path.len() - 1 {
                    let start = camera.scene_to_screen_pos(path[i].position);
                    let end = camera.scene_to_screen_pos(path[i + 1].position);
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

pub fn render_station(ui: &mut Ui, camera: &Camera, station: Station) {
    let painter = ui.painter();
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

pub fn render_stations(ui: &mut Ui, camera: &Camera, stations: &Vec<Station>) {
    for station in stations {
        render_station(ui, camera, station.clone());
    }
}

pub fn render_spawn_area(ui: &mut Ui, camera: &Camera, spawn_area: &SpawnArea) {
    let painter = ui.painter();
    let ltp = spawn_area.left_top_pos;
    let rtp = ltp + (Vec2::X * spawn_area.width).rotate(spawn_area.angle);
    let rbp = rtp + (Vec2::Y * spawn_area.height).rotate(spawn_area.angle);
    let lbp = ltp + (Vec2::Y * spawn_area.height).rotate(spawn_area.angle);
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

pub fn render_field_config(ui: &mut Ui, camera: &Camera, config: &FieldConfig) {
    let painter = ui.painter();
    for config_variant in &config.configs {
        match config_variant {
            VariantFieldConfig::Line(config) => {
                for i in 0..config.n_lines {
                    let p1 = config.left_top_pos + Vec2::new( i as f32*config.line_spacing.to_base_unit(), 0.0).rotate(config.angle);
                    let p2 = p1 + Vec2::new(0.0, config.length.to_base_unit()).rotate(config.angle);
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
                        let pos = config.left_top_pos + Vec2::new(config.line_spacing.to_base_unit()*i as f32, config.point_spacing.to_base_unit()*j as f32).rotate(config.angle);
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

pub fn render_task_manager_on_field(ui: &mut Ui, camera: &Camera, task_manager: &TaskManager) {
    let assigned_color = Color32::LIGHT_BLUE;
    let todo_color = Color32::ORANGE;
    for task in &task_manager.work_list {
        render_task(ui, camera, task, todo_color);
    }
    for task in &task_manager.assigned_tasks {
        render_task(ui, camera, task, assigned_color);
    }

    fn render_task(ui: &mut Ui, camera: &Camera, task: &Task, color: Color32) {
        let painter = ui.painter();
        match task {
            Task::Stationary { pose, .. } => {
                painter.add(CircleShape {
                    center: camera.scene_to_screen_pos(pose.position),
                    radius: camera.scene_to_screen_val(0.1),
                    fill: color,
                    stroke: Stroke::default(),
                });
            },
            Task::Moving { path, .. } => {
                let path: Vec<Pos2> = path.iter().map(|pose| camera.scene_to_screen_pos(pose.position)).collect();
                path.windows(2).for_each(|window| {
                    if let [p1, p2] = window {
                        painter.line(
                            vec![*p1, *p2], 
                            Stroke::new(camera.scene_to_screen_val(0.05), color),
                        );
                    }
                });
            },
            _ => {}
        }
    }
}

// endregion

// region: UI

pub fn ui_render_agents(ui: &mut Ui, agents: &Vec<Agent>, show_battery_plot: bool) {
    ui.label("Agents");
    Grid::new("agents")
    .striped(true)
    .show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position            ");
        ui.label("Direction         ");
        ui.label("State             ");
        // ui.label("Battery  ");
        ui.label("Current task");
        ui.label("Work Schedule");
        ui.label("Battery");
        ui.end_row();
        
        for agent in agents {
            ui.label(RichText::new("⏺").color(agent.color)); //⏹⏺
            ui.label(agent.id.to_string());
            ui.label(agent.pose.position.fmt(2));
            ui.label(agent.pose.direction.fmt(2));
            ui.label(format!("{:?}", agent.state));
            match &agent.current_task {
                Some(task) => {
                    ui.label(format!("{:?}", task.get_intent()));
                },
                None => { ui.label("False"); }
            }
            ui.label(agent.work_schedule.len().to_string());
            if !show_battery_plot {
                ui.label(format!("{:.2}%",agent.battery.get_soc()));
            } else {
                let points: PlotPoints = agent.battery.soc_history
                    .iter()
                    .enumerate()
                    .map(|(i, y)| [i as f64, f64::from(*y)])
                    .collect::<Vec<_>>()
                    .into();
    
                let line = Line::new("soc", points);
    
                Plot::new(format!("battery_plot_{}", agent.id))
                    .auto_bounds(false)
                    .show_x(false)
                    .show_y(false)
                    .allow_boxed_zoom(false)
                    .allow_double_click_reset(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .show_axes([false; 2])
                    .height(50.0)
                    .width(100.0)
                    .default_y_bounds(0.0, 100.0)
                    .default_x_bounds(0.0, 100.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
    
                        
                        let text = Text::new("soc", PlotPoint::new(3, 0), format!("{:.2}", agent.battery.soc))
                            .anchor(Align2::LEFT_BOTTOM)
                            .color(egui::Color32::LIGHT_RED);
                        plot_ui.text(text);
                        
                    });
            }


            ui.end_row();
        }
    });
}

pub fn ui_render_agents_path(ui: &mut Ui, agents: &Vec<Agent>) {
    ui.label("Agents");
    Grid::new("agents")
    .striped(true)
    .show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position            ");
        ui.label("Direction         ");
        ui.label("State             ");
        ui.label("Battery  ");
        ui.label("Path");
        ui.end_row();
        
        for agent in agents {
            ui.label(RichText::new("⏺").color(agent.color)); //⏹⏺
            ui.label(agent.id.to_string());
            ui.label(agent.pose.position.fmt(2));
            ui.label(agent.pose.direction.fmt(2));
            ui.label(format!("{:?}", agent.state));
            ui.label(format!("{:.2}", agent.battery.soc));
            let mut path_str = String::new();
            if let Some(task) = &agent.current_task {
                if let Some(path) = task.get_path() {
                    for p in path {
                        path_str += &format!("{:?}",p.position);
                    }
                }
            } else {
                path_str = "None".to_string();
            }
            ui.label(format!("Path: {}", path_str));
            ui.end_row();
        }
    });
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
                    ui.label("Power");
                    ui.label("Info");
                    ui.end_row();

                    struct TaskInfo {
                        id: u32, task_type: String, path: Vec<String>, vel: String, dur: String, fid: u32, lid: u32, power: Power, info: String
                    }

                    fn display_task_info(ui: &mut Ui, task_info: TaskInfo) {
                        ui.label(task_info.id.to_string());
                        ui.label(task_info.task_type);
                        ui.label(task_info.path.join(", "));
                        ui.label(task_info.vel);
                        ui.label(task_info.dur);
                        ui.label(task_info.fid.to_string());
                        ui.label(task_info.lid.to_string());
                        ui.label(format!("{}", task_info.power));
                        ui.label(task_info.info);
                        ui.end_row();
                    }

                    for task in vec {
                        match task {
                            Task::Stationary { id, field_id, line_id, pose, duration, power , info,..} => {
                                let task_type = "Stationary";
                                let path = vec![pose.position.fmt(2)];
                                let vel = "-".to_string();
                                let dur = format!("{}", duration);
                                let fid = field_id;
                                let lid = line_id;
                                
                                display_task_info(ui, TaskInfo { id, task_type: task_type.to_string(), path, vel, dur, fid, lid, power, info } );
                            }
                            Task::Moving { id, field_id, farm_entity_id, path, velocity, power ,info,..} => {
                                let task_type = "Moving";
                                let path: Vec<String> = path.iter()
                                    .map(|pose| pose.position.fmt(2))
                                    .collect();
                                let vel = format!("{}", velocity);
                                let dur = "-".to_string();
                                let fid = field_id;
                                let lid = farm_entity_id;
                                
                                display_task_info(ui, TaskInfo { id, task_type: task_type.to_string(), path, vel, dur, fid, lid, power, info } );
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

pub fn ui_render_datetime(ui: &mut Ui, datetime_manager: &DateTimeManager) {
    ui.label(datetime_manager.get_time());
}

// endregion
