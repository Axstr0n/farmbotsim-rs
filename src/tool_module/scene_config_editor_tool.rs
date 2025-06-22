use egui::{Slider, Ui};


use crate::cfg::{DEFAULT_SCENE_CONFIG_PATH, FIELD_CONFIGS_PATH, SCENE_CONFIGS_PATH};
use crate::environment::field_config::FieldConfig;
use crate::environment::scene_config::SceneConfig;
use crate::environment::spawn_area_module::spawn_area::SpawnArea;
use crate::environment::station_module::station_config::StationConfig;
use crate::path_finding_module::visibility_graph::VisibilityGraph;
use crate::rendering::render::{render_station};
use crate::tool_module::has_camera::HasCamera;
use crate::tool_module::has_config_saving::HasConfigSaving;
use crate::utilities::utils::{generate_colors, json_config_combo, load_json_or_panic};
use crate::{
    environment::{
        station_module::station::Station
    }, rendering::{
        camera::Camera,
        render::{render_coordinate_system, render_drag_points, render_field_config, render_grid, render_obstacles, render_spawn_area, render_visibility_graph},
    }, tool_module::{has_help::HasHelp, tool::Tool}, utilities::{pos2::ExtendedPos2}
};


pub struct SceneConfigEditorTool {
    scene_config: SceneConfig,
    field_config: FieldConfig,
    pub camera: Camera,
    save_file_name: String,
    pub current_scene_config_path: String,
    pub help_open: bool,
}

impl Default for SceneConfigEditorTool {
    fn default() -> Self {
        let scene_config: SceneConfig = load_json_or_panic(DEFAULT_SCENE_CONFIG_PATH);
        let mut field_config: FieldConfig = load_json_or_panic(scene_config.field_config_path.clone());
        field_config.recalc_id_color();
        Self {
            field_config,
            scene_config,
            camera: Camera::default(),
            save_file_name: String::new(),
            current_scene_config_path: DEFAULT_SCENE_CONFIG_PATH.to_string(),
            help_open: false,
        }
    }
}

impl Tool for SceneConfigEditorTool {
    fn update(&mut self) {}

    fn render_main(&mut self, ui: &mut Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_spawn_area(ui, &self.camera, &SpawnArea::from_config(self.scene_config.spawn_area_config.clone()));
        render_obstacles(ui, &self.camera, &self.field_config.get_obstacles());
        render_visibility_graph(ui, &self.camera, &VisibilityGraph::new(&self.field_config.get_graph_points(), self.field_config.get_obstacles()));
        render_field_config(ui, &self.camera, &self.field_config);
        let colors = generate_colors(self.scene_config.station_configs.len(), 0.0);
        for (i, station_config) in self.scene_config.station_configs.iter().enumerate() {
            render_station(ui, &self.camera, Station::from_config(0, colors[i], station_config.clone()), true);
        }
        self.handle_dragging(ui);
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        self.render_help_button(ui);
        ui.separator();

        self.ui_scene_config_select(ui);
        
        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name, true);
        self.save_file_name = save_file_name;
        ui.separator();
        
        self.ui_mouse_position(ui);
        ui.separator();
        
        self.ui_field_config_select(ui);


        ui.label(egui::RichText::new("Spawn area:").size(16.0));
        ui.label(format!("Left top pos {}", self.scene_config.spawn_area_config.left_top_pos.fmt(2)));
        ui.add(Slider::new(
            &mut self.scene_config.spawn_area_config.angle.value,
            0.0..=360.0)
            .text(format!("Angle [{}]", self.scene_config.spawn_area_config.angle.unit))
            .step_by(1.0)
        );
        ui.add(Slider::new(
            &mut self.scene_config.spawn_area_config.height.value,
            1.0..=10.0)
            .text(format!("Height [{}]", self.scene_config.spawn_area_config.height.unit))
            .step_by(0.1)
        );
        ui.add(Slider::new(
            &mut self.scene_config.spawn_area_config.width.value,
            1.0..=10.0)
            .text(format!("Width [{}]", self.scene_config.spawn_area_config.width.unit))
            .step_by(0.1)
        );

        ui.label(egui::RichText::new(format!("Stations ({}):", self.scene_config.station_configs.len())).size(16.0));
        ui.horizontal_top(|ui| {
            if ui.button("Add station").clicked() {
                self.scene_config.station_configs.push(StationConfig::default());
            }
            if ui.button("Remove all").clicked() {
                self.scene_config.station_configs.clear();
            }
        });
        
        let colors = generate_colors(self.scene_config.station_configs.len(), 0.0);
        let mut to_remove: Option<usize> = None;

        for (i, station) in self.scene_config.station_configs.iter_mut().enumerate() {
            egui::CollapsingHeader::new({
                let mut job = egui::text::LayoutJob::default();
                    job.append(
                        "⏺",
                        0.0,
                        egui::TextFormat {
                            color: colors[i],
                            ..Default::default()
                        },
                    );
                    job.append(
                        format!(" {}", i).as_str(),
                        0.0,
                        egui::TextFormat::default(),
                    );
                    job
            })
            .default_open(false)
            .show(ui, |ui| {
                ui.label(format!("Position {}", station.pose.position.fmt(2)));
                if ui.add(Slider::new(
                    &mut station.pose.orientation.value,
                    0.0..=360.0)
                    .text(format!("orientation [{}]", station.pose.orientation.unit))
                    .step_by(1.0)
                ).changed() {station.update_slots_pose();}
                ui.add(Slider::new(
                    &mut station.queue_direction.value,
                    0.0..=360.0)
                    .text(format!("queue_direction [{}]", station.queue_direction.unit))
                    .step_by(1.0)
                );
                ui.add(Slider::new(
                    &mut station.waiting_offset.value,
                    0.1..=3.0)
                    .text("waiting_offset [m]")
                    .step_by(0.1)
                );
                if ui.add(Slider::new(
                    &mut station.n_slots,
                    1..=3)
                    .text("n_slots")
                    .step_by(1.0)
                ).changed() {station.update_slots_pose();}
                ui.label("slots_pose:");
                for (i, pose) in station.slots_pose.iter_mut().enumerate() {
                    egui::CollapsingHeader::new(format!("slot_pose_{}", i))
                        .default_open(false)
                        .show(ui, |ui| {
                                ui.label(format!("Position {}", pose.position.fmt(2)));
                                ui.add(Slider::new(
                                    &mut pose.orientation.value,
                                    0.0..=360.0)
                                    .text(format!("angle [{}]", pose.orientation.unit))
                                    .step_by(1.0)
                                );
                        });
                }
                
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            });
        }

        if let Some(index) = to_remove {
            self.scene_config.station_configs.remove(index);
        }

        self.render_help(ui);
    }
    
}

impl SceneConfigEditorTool {
    fn handle_dragging(&mut self, ui: &mut Ui) {
        
        let mut pts = vec![];
        for station_config in &mut self.scene_config.station_configs {
            let screen_pos = self.camera.scene_to_screen_pos(station_config.pose.position);
            pts.push(screen_pos);
        }
        let spawn_pos = self.camera.scene_to_screen_pos(self.scene_config.spawn_area_config.left_top_pos);
        pts.push(spawn_pos);

        render_drag_points(ui, &self.camera, &pts);

        let drag_point_size = self.camera.scene_to_screen_val(0.1);
        // Drag stations
        for (i, station_config) in &mut self.scene_config.station_configs.iter_mut().enumerate() {
            let screen_pos = self.camera.scene_to_screen_pos(station_config.pose.position);
            let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
            let response = ui.interact(rect, ui.make_persistent_id(format!("station_drag_{}", i)), egui::Sense::click_and_drag());
            
            if response.dragged() {
                let drag_delta = response.drag_delta();
                let new_screen_pos = screen_pos + drag_delta;
                let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
                station_config.pose.position = new_scene_pos;
            }
        }

        // Drag spawn
        let screen_pos = self.camera.scene_to_screen_pos(self.scene_config.spawn_area_config.left_top_pos);
        let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
        let response = ui.interact(rect, ui.make_persistent_id("spawn_drag"), egui::Sense::click_and_drag());

        if response.dragged() {
            let drag_delta = response.drag_delta();
            let new_screen_pos = screen_pos + drag_delta;
            let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
            self.scene_config.spawn_area_config.left_top_pos = new_scene_pos;
        }

    }

    fn change_scene_config(&mut self) {
        let scene_config: SceneConfig = load_json_or_panic(self.current_scene_config_path.clone());
        self.scene_config = scene_config;
        self.field_config = load_json_or_panic(self.scene_config.field_config_path.clone());
    }
    fn change_field_config(&mut self) {
        self.field_config = load_json_or_panic(self.scene_config.field_config_path.clone());
        self.field_config.recalc_id_color();
    }

    fn ui_scene_config_select(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Scene config:").size(16.0));

        let mut new_value = self.current_scene_config_path.clone();

        if json_config_combo(ui, "", &mut new_value, SCENE_CONFIGS_PATH)
            && new_value != self.current_scene_config_path
        {
            self.current_scene_config_path = new_value;
            self.change_scene_config();
        }
    }

    fn ui_field_config_select(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Field config:").size(16.0));

        let mut new_value = self.scene_config.field_config_path.clone();

        if json_config_combo(ui, "  ", &mut new_value, FIELD_CONFIGS_PATH)
            && new_value != self.scene_config.field_config_path
        {
            self.scene_config.field_config_path = new_value;
            self.change_field_config();
        }
    }
}

impl HasConfigSaving for SceneConfigEditorTool {
    fn base_path() -> &'static str {
        SCENE_CONFIGS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        self.scene_config.clone()
    }
    fn update_current_path(&mut self, path: String) {
        self.current_scene_config_path = path;
    }
    fn update_after_save(&mut self) {
        self.change_scene_config();
    }
}

impl HasHelp for SceneConfigEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Scene Config Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Scene Config Editor Tool Help");
        ui.label("This is a Scene Config Editor Tool where you can create, adjust and save scene config.");
        ui.separator();

        ui.label("Scene config:");
        ui.label("In dropdown you can select scene config and save new config");
        ui.separator();

        ui.label("Mouse position:");
        ui.label("See where mouse is on screen and in env/scene.");
        ui.separator();

        ui.label("Field Config:");
        ui.label("Select field config (see FieldConfigEditor)");
        ui.separator();

        ui.label("SpawnArea Config:");
        ui.label("Set params for spawn area");
        ui.separator();

        ui.label("Station Configs:");
        ui.label("Set number and params for stations");
        ui.separator();

    }
}