use std::borrow::Borrow;

use egui::epaint::CircleShape;
use egui::{Align2, Color32, Grid, Pos2, RichText, Shape, Stroke, Ui, Vec2};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints, Text};

use crate::rendering::camera::Camera;
use farmbotsim_core::prelude::*;

// region: SCENE

/// Renders the coordinate system axes (X in red, Y in green) based on the camera view.
pub fn render_coordinate_system(ui: &mut Ui, camera: &Camera) {
    let painter = ui.painter();
    let width = camera.scene_to_screen_val(0.05);
    let stroke_x = Stroke::new(width, Color32::from_rgb(255, 0, 0));
    let stroke_y = Stroke::new(width, Color32::from_rgb(0, 255, 0));

    let zero = camera.scene_to_screen_pos(Pos2::ZERO);
    let xp = camera.scene_to_screen_pos(Pos2::new(1.0, 0.0));
    let yp = camera.scene_to_screen_pos(Pos2::new(0.0, 1.0));
    // Draw X axis
    let points_x = [zero, xp];
    painter.line_segment(points_x, stroke_x);

    // Draw Y axis
    let points_y = [zero, yp];
    painter.line_segment(points_y, stroke_y);
}

/// Draws a grid of lines over the scene within a fixed range using the camera projection.
pub fn render_grid(ui: &mut Ui, camera: &Camera) {
    let min = -100;
    let max = 100;
    let painter = ui.painter();
    let color = Color32::from_rgba_unmultiplied(255, 255, 255, 10);
    let stroke = Stroke::new(0.05, color);
    for x in min..=max {
        let p1 = camera.scene_to_screen_pos(Pos2::new(x as f32, min as f32));
        let p2 = camera.scene_to_screen_pos(Pos2::new(x as f32, max as f32));
        let points = [p1, p2];
        painter.line_segment(points, stroke);
    }
    for y in min..=max {
        let p1 = camera.scene_to_screen_pos(Pos2::new(min as f32, y as f32));
        let p2 = camera.scene_to_screen_pos(Pos2::new(max as f32, y as f32));
        let points = [p1, p2];
        painter.line_segment(points, stroke);
    }
}

/// Renders polygonal obstacles.
pub fn render_obstacles(ui: &mut Ui, camera: &Camera, obstacles: &Vec<Obstacle>) {
    let painter = ui.painter();
    for obs in obstacles {
        let points: Vec<Pos2> = obs
            .points
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

/// Draws agents with orientation indicators and their current task paths if any.
pub fn render_agents(ui: &mut Ui, camera: &Camera, agents: &Vec<Agent>) {
    let painter = ui.painter();
    let radius = camera.scene_to_screen_val(0.15);
    let length = camera.scene_to_screen_val(0.5);
    for agent in agents {
        let center = camera.scene_to_screen_pos(agent.pose.position);
        painter.add(CircleShape {
            center,
            radius,
            fill: agent.color,
            stroke: Stroke::default(),
        });
        let start = center;
        let end = center + (agent.pose.orientation.to_vec2() * Vec2::new(1.0, -1.0)) * length;
        let stroke = Stroke {
            width: camera.scene_to_screen_val(0.02),
            color: agent.color,
        };
        painter.line(vec![start, end], stroke);

        painter.add(CircleShape {
            center: end,
            radius: radius * 0.5,
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

/// Visualizes the edges and nodes of the visibility graph.
pub fn render_visibility_graph(ui: &mut Ui, camera: &Camera, visibility_graph: &VisibilityGraph) {
    // Draw edges
    let line_width = camera.scene_to_screen_val(0.05);
    let stroke = Stroke::new(
        line_width,
        Color32::from_rgba_unmultiplied(100, 100, 255, 50),
    );
    for edge in visibility_graph.graph.edge_references() {
        let (a, b) = (
            petgraph::visit::EdgeRef::source(&edge),
            petgraph::visit::EdgeRef::target(&edge),
        );
        let start = camera.scene_to_screen_pos(visibility_graph.graph[a]);
        let end = camera.scene_to_screen_pos(visibility_graph.graph[b]);
        ui.painter().line_segment([start, end], stroke);
    }

    // Draw nodes on top
    let node_radius = camera.scene_to_screen_val(0.01);
    for node in visibility_graph.graph.node_indices() {
        let pos = camera.scene_to_screen_pos(visibility_graph.graph[node]);
        ui.painter().circle_filled(pos, node_radius, Color32::RED);
    }
}

/// Renders a station with slots and optionally visualizes its parameters.
pub fn render_station(ui: &mut Ui, camera: &Camera, station: &Station, with_params: bool) {
    let painter = ui.painter();
    let center = camera.scene_to_screen_pos(station.pose.position);

    // Scene dimensions
    let width = match station.n_slots {
        1 => 0.4,
        2 => 0.7,
        3 => 1.0,
        _ => 100.0,
    };
    let outer_width = camera.scene_to_screen_val(width);
    let outer_height = camera.scene_to_screen_val(0.4);
    let inner_width = camera.scene_to_screen_val(width - 0.1);
    let inner_height = camera.scene_to_screen_val(0.3);

    let angle = -station.pose.orientation.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    // Define corners in local space (centered at origin, from -0.5 to 0.5)
    let rect_corners = [
        Vec2::new(-0.5, -0.5),
        Vec2::new(0.5, -0.5),
        Vec2::new(0.5, 0.5),
        Vec2::new(-0.5, 0.5),
    ];

    // Helper to rotate and scale corners
    fn transform_corners(
        corners: &[Vec2; 4],
        center: Pos2,
        width: f32,
        height: f32,
        cos_a: f32,
        sin_a: f32,
    ) -> Vec<Pos2> {
        corners
            .iter()
            .map(|corner| {
                let scaled = Vec2::new(corner.x * width, corner.y * height);
                let rotated = Vec2::new(
                    scaled.x * cos_a - scaled.y * sin_a,
                    scaled.x * sin_a + scaled.y * cos_a,
                );
                center + rotated
            })
            .collect()
    }

    // Draw outer rectangle (border)
    let outer_rect = transform_corners(
        &rect_corners,
        center,
        outer_width,
        outer_height,
        cos_a,
        sin_a,
    );
    painter.add(Shape::convex_polygon(
        outer_rect,
        station.color,
        Stroke::default(),
    ));

    // Draw inner rectangle (background)
    let inner_rect = transform_corners(
        &rect_corners,
        center,
        inner_width,
        inner_height,
        cos_a,
        sin_a,
    );
    painter.add(Shape::convex_polygon(
        inner_rect,
        Color32::BLACK,
        Stroke::default(),
    ));

    if with_params {
        // queue direction
        let end = camera.scene_to_screen_pos(station.get_waiting_pose(0).position);
        let stroke = Stroke {
            width: camera.scene_to_screen_val(0.02),
            color: Color32::MAGENTA,
        };
        painter.line(vec![center, end], stroke);
        let radius = camera.scene_to_screen_val(0.05);
        painter.add(CircleShape {
            center: end,
            radius,
            fill: Color32::from_rgba_premultiplied(255, 0, 0, 150),
            stroke: Stroke::default(),
        });
    }
    // slots
    let length = camera.scene_to_screen_val(0.5);
    let line_width = camera.scene_to_screen_val(0.02);
    let radius = camera.scene_to_screen_val(0.1);
    let stroke = Stroke {
        width: line_width,
        color: Color32::YELLOW,
    };
    for i in 0..station.slots_pose.len() {
        let pose = station.get_pose_for_slot(i);
        if let Some(pose) = pose {
            let center = camera.scene_to_screen_pos(pose.position);
            if with_params {
                // slot orientation
                let end = center + (pose.orientation.to_vec2() * Vec2::new(1.0, -1.0)) * length;
                painter.line(vec![center, end], stroke);
            }
            // slot position
            let color = if station.slots[i].is_some() {
                Color32::RED
            } else {
                Color32::LIGHT_BLUE
            };
            painter.add(CircleShape {
                center,
                radius,
                fill: color,
                stroke: Stroke::default(),
            });
        }
    }
}

/// Renders multiple stations by calling `render_station` for each.
pub fn render_stations(ui: &mut Ui, camera: &Camera, stations: &Vec<Station>, with_params: bool) {
    for station in stations {
        render_station(ui, camera, station, with_params);
    }
}

/// Draws the spawn area as a rotated translucent polygon.
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

/// Renders red draggable points at specified positions.
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

/// Visualizes field configuration elements such as lines or points based on the config variants.
pub fn render_field_config(ui: &mut Ui, camera: &Camera, config: &FieldConfig) {
    let painter = ui.painter();
    for config_variant in &config.configs {
        match config_variant {
            VariantFieldConfig::Line(config) => {
                for i in 0..config.n_lines {
                    let p1 = config.left_top_pos
                        + Vec2::new(i as f32 * config.line_spacing.to_base_unit(), 0.0)
                            .rotate(config.angle);
                    let p2 = p1 + Vec2::new(0.0, config.length.to_base_unit()).rotate(config.angle);
                    let p1 = camera.scene_to_screen_pos(p1);
                    let p2 = camera.scene_to_screen_pos(p2);
                    let stroke = Stroke {
                        width: camera.scene_to_screen_val(0.02),
                        color: config.color,
                    };
                    painter.line(vec![p1, p2], stroke);
                }
            }
            VariantFieldConfig::Point(config) => {
                for i in 0..config.n_lines {
                    for j in 0..config.n_points_per_line {
                        let pos = config.left_top_pos
                            + Vec2::new(
                                config.line_spacing.to_base_unit() * i as f32,
                                config.point_spacing.to_base_unit() * j as f32,
                            )
                            .rotate(config.angle);
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

/// Draws tasks from the task manager on the field with different colors based on their state.
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
            }
            Task::Moving { path, .. } => {
                let path: Vec<Pos2> = path
                    .iter()
                    .map(|pose| camera.scene_to_screen_pos(pose.position))
                    .collect();
                path.windows(2).for_each(|window| {
                    if let [p1, p2] = window {
                        painter.line(
                            vec![*p1, *p2],
                            Stroke::new(camera.scene_to_screen_val(0.05), color),
                        );
                    }
                });
            }
            _ => {}
        }
    }
}

// endregion

// region: UI

/// Displays a grid listing agents with their properties and optionally a battery plot.
pub fn ui_render_agents(ui: &mut Ui, agents: &Vec<Agent>, show_battery_plot: bool) {
    ui.label("Agents");
    Grid::new("agents").striped(true).show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position            ");
        ui.label("Orientation         ");
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
            ui.label(format!(
                "{}°",
                agent.pose.orientation.to_degrees().round() as i32
            ));
            ui.label(format!("{:?}", agent.state));
            match &agent.current_task {
                Some(task) => {
                    ui.label(format!("{:?}", task.get_intent()));
                }
                None => {
                    ui.label("False");
                }
            }
            ui.label(agent.work_schedule.len().to_string());
            if !show_battery_plot {
                ui.label(format!("{:.2}%", agent.battery.get_soc()));
            } else {
                let points: PlotPoints = agent
                    .battery
                    .soc_history
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

                        let text = Text::new(
                            "soc",
                            PlotPoint::new(3, 0),
                            format!("{:.2}", agent.battery.soc),
                        )
                        .anchor(Align2::LEFT_BOTTOM)
                        .color(egui::Color32::LIGHT_RED);
                        plot_ui.text(text);
                    });
            }

            ui.end_row();
        }
    });
}

/// Shows agents and their current paths.
pub fn ui_render_agents_path(ui: &mut Ui, agents: &Vec<Agent>) {
    ui.label("Agents");
    Grid::new("agents").striped(true).show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position            ");
        ui.label("Orientation         ");
        ui.label("State             ");
        ui.label("Battery  ");
        ui.label("Path");
        ui.end_row();

        for agent in agents {
            ui.label(RichText::new("⏺").color(agent.color)); //⏹⏺
            ui.label(agent.id.to_string());
            ui.label(agent.pose.position.fmt(2));
            ui.label(format!(
                "{}°",
                agent.pose.orientation.to_degrees().round() as i32
            ));
            ui.label(format!("{:?}", agent.state));
            ui.label(format!("{:.2}", agent.battery.soc));
            let mut path_str = String::new();
            if let Some(task) = &agent.current_task {
                if let Some(path) = task.get_path() {
                    for p in path {
                        path_str += &format!("{:?}", p.position);
                    }
                }
            } else {
                path_str = "None".to_string();
            }
            ui.label(format!("Path: {path_str}"));
            ui.end_row();
        }
    });
}

/// Presents a grid UI listing stations with details such as position, slots, and queue length.
pub fn ui_render_stations(ui: &mut Ui, stations: &Vec<Station>) {
    ui.label("Stations");
    Grid::new("stations").striped(true).show(ui, |ui| {
        ui.label(" ");
        ui.label("Id");
        ui.label("Position");
        ui.label("Orientation");
        ui.label("N slots");
        ui.label("Occupied slots");
        ui.label("Queue");
        ui.end_row();

        for station in stations {
            ui.label(RichText::new("⏺").color(station.color)); //⏹⏺
            ui.label(station.id.to_string());
            ui.label(station.pose.position.fmt(2));
            ui.label(format!(
                "{}°",
                station.pose.orientation.to_degrees().round() as i32
            ));
            ui.label(station.n_slots.to_string());
            ui.label(station.n_occupied_slots().to_string());
            ui.label(station.queue.len().to_string());
            ui.end_row();
        }
    });
}

/// Displays the task manager's work, assigned, and completed tasks in collapsible grids.
pub fn ui_render_task_manager(ui: &mut Ui, task_manager: &TaskManager) {
    fn make_grid_from<I>(ui: &mut Ui, label: String, iterator: I)
    where
        I: IntoIterator,
        I::Item: Borrow<Task>,
    {
        let tasks: Vec<_> = iterator.into_iter().collect();
        ui.collapsing(format!("{} ({})", label, tasks.len()), |ui| {
            Grid::new(label.to_string()).striped(true).show(ui, |ui| {
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

                struct TaskInfo<'a> {
                    id: u32,
                    task_type: &'a str,
                    path: &'a [String],
                    vel: &'a str,
                    dur: &'a str,
                    fid: u32,
                    lid: u32,
                    power: Power,
                    info: &'a str,
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

                for task in tasks {
                    let task = task.borrow();
                    match task {
                        Task::Stationary {
                            id,
                            field_id,
                            line_id,
                            pose,
                            duration,
                            power,
                            info,
                            ..
                        } => {
                            let task_type = "Stationary";
                            let path = vec![pose.position.fmt(2)];
                            let vel = "-".to_string();
                            let dur = format!("{duration}");
                            let fid = field_id;
                            let lid = line_id;

                            display_task_info(
                                ui,
                                TaskInfo {
                                    id: *id,
                                    task_type,
                                    path: &path,
                                    vel: &vel,
                                    dur: &dur,
                                    fid: *fid,
                                    lid: *lid,
                                    power: *power,
                                    info,
                                },
                            );
                        }
                        Task::Moving {
                            id,
                            field_id,
                            farm_entity_id,
                            path,
                            velocity,
                            power,
                            info,
                            ..
                        } => {
                            let task_type = "Moving";
                            let path: Vec<String> =
                                path.iter().map(|pose| pose.position.fmt(2)).collect();
                            let vel = format!("{velocity}");
                            let dur = "-".to_string();
                            let fid = field_id;
                            let lid = farm_entity_id;

                            display_task_info(
                                ui,
                                TaskInfo {
                                    id: *id,
                                    task_type,
                                    path: &path,
                                    vel: &vel,
                                    dur: &dur,
                                    fid: *fid,
                                    lid: *lid,
                                    power: *power,
                                    info,
                                },
                            );
                        }
                        _ => {}
                    }
                }
            });
        });
    }

    ui.label("Task manager");
    make_grid_from(ui, "Work List".to_string(), &task_manager.work_list);
    make_grid_from(
        ui,
        "Assigned List".to_string(),
        &task_manager.assigned_tasks,
    );
    make_grid_from(
        ui,
        "Completed List".to_string(),
        &task_manager.completed_tasks,
    );
}

/// Shows the current date and time from the `DateTimeManager`.
pub fn ui_render_datetime(ui: &mut Ui, datetime_manager: &DateTimeManager) {
    ui.label(datetime_manager.get_time());
}

// endregion
