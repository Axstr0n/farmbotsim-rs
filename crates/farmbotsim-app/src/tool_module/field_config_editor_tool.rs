use egui::{Slider, Ui};

use crate::{
    rendering::{
        camera::Camera,
        render::{
            render_coordinate_system, render_drag_points, render_field_config, render_grid,
            render_obstacles, render_visibility_graph,
        },
    },
    tool_module::{
        has_camera::HasCamera, has_config_saving::HasConfigSaving, has_help::HasHelp, tool::Tool,
    },
    utilities::{files::get_json_files_in_folder, ui::json_config_combo},
};

use farmbotsim_core::prelude::*;

/// A tool for editing, viewing, changing field configuration
pub struct FieldConfigEditorTool {
    pub field_config: FieldConfig,
    pub camera: Camera,
    save_file_name: String,
    pub current_field_config_path: String,
    pub help_open: bool,
}

impl Default for FieldConfigEditorTool {
    fn default() -> Self {
        let mut field_config: FieldConfig = load_json_or_panic(DEFAULT_FIELD_CONFIG_PATH);
        field_config.recalc_id_color();
        Self {
            field_config,
            camera: Camera::default(),
            save_file_name: String::new(),
            current_field_config_path: DEFAULT_FIELD_CONFIG_PATH.to_string(),
            help_open: false,
        }
    }
}

impl Tool for FieldConfigEditorTool {
    fn update(&mut self) {}

    fn render_main(&mut self, ui: &mut Ui) {
        self.camera.handle_events(ui);
        render_grid(ui, &self.camera);
        render_coordinate_system(ui, &self.camera);
        render_obstacles(ui, &self.camera, &self.field_config.get_obstacles());
        render_visibility_graph(
            ui,
            &self.camera,
            &VisibilityGraph::new(
                &self.field_config.get_graph_points(),
                self.field_config.get_obstacles(),
            ),
        );
        render_field_config(ui, &self.camera, &self.field_config);
        self.handle_dragging(ui);
    }

    fn render_ui(&mut self, ui: &mut Ui) {
        self.render_help_button(ui);
        ui.separator();

        self.ui_field_config_select(ui);

        let mut save_file_name = self.save_file_name.clone();
        self.draw_save_ui(ui, &mut save_file_name, true);
        self.save_file_name = save_file_name;
        ui.separator();

        self.ui_mouse_position(ui);
        ui.separator();

        ui.label(egui::RichText::new("Fields:").size(16.0));

        ui.horizontal_top(|ui| {
            if ui.button("Add line").clicked() {
                self.field_config
                    .configs
                    .push(VariantFieldConfig::Line(LineFieldConfig::default()));
                self.field_config.recalc_id_color();
            }
            if ui.button("Add point").clicked() {
                self.field_config
                    .configs
                    .push(VariantFieldConfig::Point(PointFieldConfig::default()));
                self.field_config.recalc_id_color();
            }
            if ui.button("Remove all").clicked() {
                self.field_config.configs.clear();
                self.field_config.recalc_id_color();
            }
        });

        let mut to_remove: Option<usize> = None;
        let mut needs_recalc = false;

        for (i, config_variant) in self.field_config.configs.iter_mut().enumerate() {
            match config_variant {
                VariantFieldConfig::Line(config) => {
                    egui::CollapsingHeader::new({
                        let mut job = egui::text::LayoutJob::default();
                        job.append(
                            "⏺",
                            0.0,
                            egui::TextFormat {
                                color: config.color,
                                ..Default::default()
                            },
                        );
                        job.append(
                            format!(" Line {i}").as_str(),
                            0.0,
                            egui::TextFormat::default(),
                        );
                        job
                    })
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.label(format!("Left top pos {}", config.left_top_pos.fmt(2)));
                        let angle_response = ui.add(
                            Slider::new(&mut config.angle.value, 0.0..=360.0)
                                .text(format!("Angle [{}]", config.angle.unit))
                                .step_by(1.0),
                        );
                        let n_lines_response = ui.add(
                            Slider::new(&mut config.n_lines, 1..=10)
                                .text("N_lines")
                                .step_by(1.0),
                        );
                        let line_spacing_response = ui.add(
                            Slider::new(&mut config.line_spacing.value, 0.2..=0.8)
                                .text(format!("Line spacing [{}]", config.line_spacing.unit))
                                .step_by(0.05),
                        );
                        let length_response = ui.add(
                            Slider::new(&mut config.length.value, 1.0..=10.0)
                                .text(format!("Length [{}]", config.length.unit))
                                .step_by(0.05),
                        );

                        egui::ComboBox::from_label("")
                            .selected_text(&config.farm_entity_plan_path)
                            .show_ui(ui, |ui| {
                                let json_files = get_json_files_in_folder(FARM_ENTITY_PLANS_PATH);
                                for json_file in json_files {
                                    let whole_path =
                                        format!("{}{}", FARM_ENTITY_PLANS_PATH, json_file.clone());
                                    let plan = FarmEntityPlan::from_json_file(&whole_path);
                                    if plan.type_.to_lowercase() == "line" {
                                        ui.selectable_value(
                                            &mut config.farm_entity_plan_path,
                                            whole_path.clone(),
                                            whole_path,
                                        );
                                    }
                                }
                            });

                        if ui.button("Remove").clicked() {
                            to_remove = Some(i);
                        }
                        if angle_response.changed()
                            || n_lines_response.changed()
                            || line_spacing_response.changed()
                            || length_response.changed()
                        {
                            needs_recalc = true;
                        }
                    });
                }
                VariantFieldConfig::Point(config) => {
                    egui::CollapsingHeader::new({
                        let mut job = egui::text::LayoutJob::default();
                        job.append(
                            "⏺",
                            0.0,
                            egui::TextFormat {
                                color: config.color,
                                ..Default::default()
                            },
                        );
                        job.append(
                            format!(" Point {i}").as_str(),
                            0.0,
                            egui::TextFormat::default(),
                        );
                        job
                    })
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.label(format!("Left top pos {}", config.left_top_pos.fmt(2)));
                        let angle_response = ui.add(
                            Slider::new(&mut config.angle.value, 0.0..=360.0)
                                .text(format!("Angle [{}]", config.angle.unit))
                                .step_by(1.0),
                        );
                        let n_lines_response = ui.add(
                            Slider::new(&mut config.n_lines, 1..=10)
                                .text("N_lines")
                                .step_by(1.0),
                        );
                        let line_spacing_response = ui.add(
                            Slider::new(&mut config.line_spacing.value, 0.2..=0.8)
                                .text(format!("Line spacing [{}]", config.line_spacing.unit))
                                .step_by(0.05),
                        );
                        let n_points_per_line_response = ui.add(
                            Slider::new(&mut config.n_points_per_line, 1..=10)
                                .text("N points per line")
                                .step_by(1.0),
                        );
                        let point_spacing_response = ui.add(
                            Slider::new(&mut config.point_spacing.value, 0.2..=0.8)
                                .text(format!("Point spacing [{}]", config.point_spacing.unit))
                                .step_by(0.05),
                        );

                        egui::ComboBox::from_label("")
                            .selected_text(&config.farm_entity_plan_path)
                            .show_ui(ui, |ui| {
                                let json_files = get_json_files_in_folder(FARM_ENTITY_PLANS_PATH);
                                for json_file in json_files {
                                    let whole_path =
                                        format!("{}{}", FARM_ENTITY_PLANS_PATH, json_file.clone());
                                    let plan = FarmEntityPlan::from_json_file(&whole_path);
                                    if plan.type_.to_lowercase() == "point" {
                                        ui.selectable_value(
                                            &mut config.farm_entity_plan_path,
                                            whole_path.clone(),
                                            whole_path,
                                        );
                                    }
                                }
                            });

                        if ui.button("Remove").clicked() {
                            to_remove = Some(i);
                        }
                        if angle_response.changed()
                            || n_lines_response.changed()
                            || line_spacing_response.changed()
                            || n_points_per_line_response.changed()
                            || point_spacing_response.changed()
                        {
                            needs_recalc = true;
                        }
                    });
                }
            }
        }

        if let Some(index) = to_remove {
            self.field_config.configs.remove(index);
            self.field_config.recalc_id_color();
        }

        if needs_recalc {
            self.field_config.recalc_id_color();
        }

        self.render_help(ui);
    }
}

impl FieldConfigEditorTool {
    /// Handles dragging field configs
    fn handle_dragging(&mut self, ui: &mut Ui) {
        let mut pts = vec![];
        for config_variant in &mut self.field_config.configs {
            let left_top_pos = match config_variant {
                VariantFieldConfig::Line(config) => &config.left_top_pos,
                VariantFieldConfig::Point(config) => &config.left_top_pos,
            };

            let pos = self.camera.scene_to_screen_pos(*left_top_pos);
            pts.push(pos);
        }

        render_drag_points(ui, &self.camera, &pts);

        let drag_point_size = self.camera.scene_to_screen_val(0.1);
        // Drag field
        for (i, config_variant) in &mut self.field_config.configs.iter_mut().enumerate() {
            let left_top_pos = match config_variant {
                VariantFieldConfig::Line(config) => &mut config.left_top_pos,
                VariantFieldConfig::Point(config) => &mut config.left_top_pos,
            };

            let screen_pos = self.camera.scene_to_screen_pos(*left_top_pos);
            let rect = egui::Rect::from_center_size(screen_pos, egui::Vec2::splat(drag_point_size));
            let response = ui.interact(
                rect,
                ui.make_persistent_id(format!("field_drag_{i}")),
                egui::Sense::click_and_drag(),
            );

            if response.dragged_by(egui::PointerButton::Primary) {
                let drag_delta = response.drag_delta();
                let new_screen_pos = screen_pos + drag_delta;
                let new_scene_pos = self.camera.screen_to_scene_pos(new_screen_pos);
                *left_top_pos = new_scene_pos;
            }
        }
    }

    /// Changes field config to new value
    fn change_field_config(&mut self, new_field_config_path: String) {
        let field_config: FieldConfig = load_json_or_panic(new_field_config_path);
        self.field_config = field_config;
    }

    /// Renders dropdown to select field configuration file
    fn ui_field_config_select(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Field config:").size(16.0));

        let mut new_value = self.current_field_config_path.clone();

        if json_config_combo(ui, "  ", &mut new_value, FIELD_CONFIGS_PATH)
            && new_value != self.current_field_config_path
        {
            self.current_field_config_path = new_value;
            self.change_field_config(self.current_field_config_path.clone());
        }
    }
}

impl HasConfigSaving for FieldConfigEditorTool {
    fn base_path() -> &'static str {
        FIELD_CONFIGS_PATH
    }
    fn config(&self) -> impl serde::Serialize {
        self.field_config.clone()
    }
    fn update_current_path(&mut self, path: String) {
        self.current_field_config_path = path;
    }
    fn update_after_save(&mut self) {
        self.change_field_config(self.current_field_config_path.clone());
    }
}

impl HasHelp for FieldConfigEditorTool {
    fn help_modal(&self) -> egui::Modal {
        egui::Modal::new(egui::Id::new("Field Config Editor Tool Help"))
    }
    fn render_help_contents(&self, ui: &mut egui::Ui) {
        ui.heading("Field Config Editor Tool Help");
        ui.label("This is a Field Config Editor Tool where you can create, adjust and save field config.");
        ui.separator();

        ui.label("Field config:");
        ui.label("In dropdown you can select field config and save new config");
        ui.separator();

        ui.label("Mouse position:");
        ui.label("See where mouse is on screen and in env/scene.");
        ui.separator();

        ui.label("Fields:");
        ui.label("Add, remove, change variant field config (Supported Line, Point).");
        ui.label("Select farm entity plan for field (see FarmEntityPlanEditor).");
    }
}
