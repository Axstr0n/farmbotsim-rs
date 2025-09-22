use eframe::egui;
use std::{time::{Duration, Instant}};

use crate::{
    app_module::app_mode::AppMode, tool_module::{
        agent_config_editor_tool::AgentConfigEditorTool, battery_tool::BatteryTool, farm_entity_plan_editor_tool::FarmEntityPlanEditorTool, field_config_editor_tool::FieldConfigEditorTool, general_help_tool::GeneralHelpTool, movement_config_editor_tool::MovementConfigEditorTool, path_tool::PathTool, performance_matrix_tool::PerformanceMatrixTool, scene_config_editor_tool::SceneConfigEditorTool, simulation_tool::SimulationTool, task_manager_config_editor_tool::TaskManagerConfigEditorTool, task_tool::TaskTool, tool::Tool
    }
};

/// The main application struct that holds state and manages tools and UI.
pub struct App {
    /// The current mode of the application.
    mode: AppMode,
    /// Whether dark mode is enabled.
    is_dark_mode: bool,

    simulation_tool: SimulationTool,
    path_tool: PathTool,
    task_tool: TaskTool,
    battery_tool: BatteryTool,
    farm_entity_plan_editor_tool: FarmEntityPlanEditorTool,
    movement_config_editor_tool: MovementConfigEditorTool,
    agent_config_editor_tool: AgentConfigEditorTool,
    field_config_editor_tool: FieldConfigEditorTool,
    scene_config_editor_tool: SceneConfigEditorTool,
    performance_matrix_tool: PerformanceMatrixTool,
    task_manager_config_editor_tool: TaskManagerConfigEditorTool,
    general_help_tool: GeneralHelpTool,

    /// Frames per second.
    fps: f32,
    /// Ticks per second.
    tps: f32,
    /// Ratio of ticks per second to frames per second.
    ratio_tps_fps: f32,

    /// Total number of ticks since app started.
    ticks: u64,
    /// Total number of frames rendered since app started.
    frames: u64,
    /// Tick count since last stats update.
    tick_count: u32,
    /// Frame count since last stats update.
    frame_count: u32,
    /// Last time FPS/TPS stats were updated.
    last_stat_time: Instant,

    /// Accumulator used to control tick timing when TPS < FPS.
    accumulator: f32,
}


impl Default for App {
    /// Creates a new `App` instance with all tools in their default state
    /// and the application set to `Simulation` mode with dark mode enabled.
    fn default() -> Self {
        Self {
            mode: AppMode::Simulation,
            is_dark_mode: true,

            simulation_tool: SimulationTool::default(),
            path_tool: PathTool::default(),
            task_tool: TaskTool::default(),
            battery_tool: BatteryTool::default(),
            farm_entity_plan_editor_tool: FarmEntityPlanEditorTool::default(),
            movement_config_editor_tool: MovementConfigEditorTool::default(),
            agent_config_editor_tool: AgentConfigEditorTool::default(),
            field_config_editor_tool: FieldConfigEditorTool::default(),
            scene_config_editor_tool: SceneConfigEditorTool::default(),
            performance_matrix_tool: PerformanceMatrixTool::default(),
            task_manager_config_editor_tool: TaskManagerConfigEditorTool::default(),
            general_help_tool: GeneralHelpTool::default(),

            fps: 0.0,
            tps: 0.0,
            ratio_tps_fps: 1.0,

            ticks: 0, // always increment
            frames: 0, // always increment
            frame_count: 0,
            tick_count: 0,
            last_stat_time: Instant::now(),

            accumulator: 0.0,
        }
    }
}


impl eframe::App for App {
    /// Called each frame by `eframe`. Updates logic and renders UI based on TPS/FPS ratio.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.ticks <= 1 {
            if self.is_dark_mode { ctx.set_visuals(egui::Visuals::dark()); }
            else { ctx.set_visuals(egui::Visuals::light()); }
        }

        let now = Instant::now();

        if self.ratio_tps_fps >= 1.0 { // More TPS than FPS
            let updates_per_frame = self.ratio_tps_fps as usize;
            for _ in 0..updates_per_frame {
                self.update_();
            }
            self.render(ctx, frame);
        }
        else { // More FPS than TPS
            self.accumulator += self.ratio_tps_fps;
            if self.accumulator >= 1.0 {
                self.update_();
                self.accumulator -= 1.0;
            }
            self.render(ctx, frame);
        }

        // Statistics (FPS/TPS)
        if now - self.last_stat_time >= Duration::from_secs(1) {
            let elapsed = (now - self.last_stat_time).as_secs_f32();
            self.fps = self.frame_count as f32 / elapsed;
            self.tps = self.tick_count as f32 / elapsed;
            self.frame_count = 0;
            self.tick_count = 0;
            self.last_stat_time = now;
        }

        ctx.request_repaint();

    }

}

impl App {
    /// Performs a single logical update (tick) for the active tool based on the current `AppMode`.
    pub fn update_(&mut self) {
        self.tick_count += 1;
        self.ticks += 1;
        match self.mode {
            AppMode::Simulation => self.simulation_tool.update(),
            AppMode::Path => self.path_tool.update(),
            AppMode::Task => self.task_tool.update(),
            AppMode::Battery => {},
            AppMode::FarmEntityPlanEditor => {},
            AppMode::MovementConfigEditor => {},
            AppMode::AgentConfigEditor => {},
            AppMode::FieldConfigEditor => {},
            AppMode::SceneConfigEditor => {},
            AppMode::PerformanceMatrix => self.performance_matrix_tool.update(),
            AppMode::TaskManagerConfigEditor => {},
            AppMode::GeneralHelp => {}
        }
        
    }
    /// Renders the full user interface based on the current application mode.
    pub fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        self.frames += 1;

        ctx.request_repaint();

        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.mode, AppMode::GeneralHelp, "GeneralHelp");
                ui.separator();
                ui.separator();
                ui.selectable_value(&mut self.mode, AppMode::MovementConfigEditor, "MovementConfigEditor");
                ui.selectable_value(&mut self.mode, AppMode::Battery, "Battery");
                ui.selectable_value(&mut self.mode, AppMode::AgentConfigEditor, "AgentConfigEditor");
                ui.separator();
                ui.selectable_value(&mut self.mode, AppMode::FarmEntityPlanEditor, "FarmEntityPlanEditor");
                ui.selectable_value(&mut self.mode, AppMode::FieldConfigEditor, "FieldConfigEditor");
                ui.selectable_value(&mut self.mode, AppMode::SceneConfigEditor, "SceneConfigEditor");
                ui.selectable_value(&mut self.mode, AppMode::TaskManagerConfigEditor, "TaskManagerConfigEditor");
                ui.separator();
                ui.selectable_value(&mut self.mode, AppMode::Simulation, "Simulation");
                ui.selectable_value(&mut self.mode, AppMode::Path, "Path");
                ui.selectable_value(&mut self.mode, AppMode::Task, "Task");
                ui.selectable_value(&mut self.mode, AppMode::PerformanceMatrix, "PerformanceMatrix");

                ui.separator();
                ui.separator();

                // Settings menu
                ui.menu_button("Settings", |ui| {
                    // Theme toggle
                    if self.is_dark_mode && ui.button("To Light mode").clicked() {
                        self.is_dark_mode = false;
                        ctx.set_visuals(egui::Visuals::light());
                    } else if !self.is_dark_mode && ui.button("To Dark mode").clicked() {
                        self.is_dark_mode = true;
                        ctx.set_visuals(egui::Visuals::dark());
                    }

                    ui.separator();

                    // TPS/FPS settings
                    ui.label("TPS/FPS ratio:");
                    ui.add(
                        egui::Slider::new(&mut self.ratio_tps_fps, 0.01..=10000.0)
                        .logarithmic(true)
                    );
                    ui.label(format!("TPS: {:.2}", self.tps));
                    ui.label(format!("FPS: {:.2}", self.fps));
                });
            });
        });

        // Side panel for stats
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(400.0)
            .min_width(350.0)
            .max_width(650.0)
            .show(ctx, |ui| {
                
                ui.label(egui::RichText::new(self.mode.to_string()).strong().size(20.0));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    
                    match self.mode {
                        AppMode::Simulation => self.simulation_tool.render_ui(ui),
                        AppMode::Path => self.path_tool.render_ui(ui),
                        AppMode::Task => self.task_tool.render_ui(ui),
                        AppMode::Battery => self.battery_tool.render_ui(ui),
                        AppMode::FarmEntityPlanEditor => self.farm_entity_plan_editor_tool.render_ui(ui),
                        AppMode::MovementConfigEditor => self.movement_config_editor_tool.render_ui(ui),
                        AppMode::AgentConfigEditor => self.agent_config_editor_tool.render_ui(ui),
                        AppMode::FieldConfigEditor => self.field_config_editor_tool.render_ui(ui),
                        AppMode::SceneConfigEditor => self.scene_config_editor_tool.render_ui(ui),
                        AppMode::PerformanceMatrix => self.performance_matrix_tool.render_ui(ui),
                        AppMode::TaskManagerConfigEditor => self.task_manager_config_editor_tool.render_ui(ui),
                        AppMode::GeneralHelp => self.general_help_tool.render_ui(ui),
                    }
                });
        });
        
        // Main scene area for (zoom/pan) or text editor
        egui::CentralPanel::default().show(ctx, |ui| {

            match self.mode {
                AppMode::Simulation | AppMode::Path | AppMode::Task | AppMode::FieldConfigEditor | AppMode::SceneConfigEditor => {
                    // Tools with camera
                    egui::Frame::group(ui.style())
                        .inner_margin(0.0)
                        .show(ui, |ui| {
                            match self.mode {
                                AppMode::Simulation => { self.simulation_tool.render_main(ui); }
                                AppMode::Path => { self.path_tool.render_main(ui); }
                                AppMode::Task => { self.task_tool.render_main(ui); }
                                AppMode::FieldConfigEditor => { self.field_config_editor_tool.render_main(ui); }
                                AppMode::SceneConfigEditor => { self.scene_config_editor_tool.render_main(ui); }
                                _ => {},
                            }
                        });
                    },
                AppMode::Battery | AppMode::FarmEntityPlanEditor | AppMode::MovementConfigEditor | AppMode::AgentConfigEditor | AppMode::PerformanceMatrix | AppMode::TaskManagerConfigEditor | AppMode::GeneralHelp => {
                    // Tools without camera
                    match self.mode {
                        AppMode::Battery => self.battery_tool.render_main(ui),
                        AppMode::FarmEntityPlanEditor => self.farm_entity_plan_editor_tool.render_main(ui),
                        AppMode::MovementConfigEditor => self.movement_config_editor_tool.render_main(ui),
                        AppMode::AgentConfigEditor => self.agent_config_editor_tool.render_main(ui),
                        AppMode::PerformanceMatrix => self.performance_matrix_tool.render_main(ui),
                        AppMode::TaskManagerConfigEditor => self.task_manager_config_editor_tool.render_main(ui),
                        AppMode::GeneralHelp => self.general_help_tool.render_main(ui),
                    _ => {}
                    }
                },
            }
        });
    }
}
