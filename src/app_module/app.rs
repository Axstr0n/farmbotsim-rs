use eframe::egui;
use std::{time::{Duration, Instant}};

use crate::{
    app_module::app_mode::AppMode, tool_module::{
        agent_config_editor_tool::AgentConfigEditorTool, battery_tool::BatteryTool, env_config_editor_tool::EnvConfigEditorTool, farm_entity_plan_editor_tool::FarmEntityPlanEditorTool, field_config_editor_tool::FieldConfigEditorTool, movement_config_editor_tool::MovementConfigEditorTool, path_tool::PathTool, performance_matrix_tool::PerformanceMatrixTool, scene_config_editor_tool::SceneConfigEditorTool, simulation_tool::SimulationTool, task_tool::TaskTool, tool::Tool
    }
};

pub struct App {
    mode: AppMode,
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
    env_config_editor_tool: EnvConfigEditorTool,
    performance_matrix_tool: PerformanceMatrixTool,

    fps: f32,
    tps: f32,
    ratio_tps_fps: f32,

    ticks: u64,
    frames: u64,
    frame_count: u32,
    tick_count: u32,
    last_stat_time: Instant,

    accumulator: f32,
}


impl Default for App {
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
            env_config_editor_tool: EnvConfigEditorTool::default(),
            performance_matrix_tool: PerformanceMatrixTool::default(),

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
            AppMode::EnvConfigEditor => {},
            AppMode::PerformanceMatrixTool => self.performance_matrix_tool.update(),
        }
        
    }
    pub fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        self.frames += 1;

        ctx.request_repaint();

        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {     
                for mode in AppMode::variants() {
                    if ui.button(mode.to_string()).clicked() {
                        println!("{}", mode.to_string());
                        self.mode = mode;
                    }         
                }
                // Spacer
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
                        AppMode::EnvConfigEditor => self.env_config_editor_tool.render_ui(ui),
                        AppMode::PerformanceMatrixTool => self.performance_matrix_tool.render_ui(ui),
                    }
                });
        });
        
        // Main scene area for (zoom/pan) or text editor
        egui::CentralPanel::default().show(ctx, |ui| {

            match self.mode {
                AppMode::Simulation | AppMode::Path | AppMode::Task | AppMode::FieldConfigEditor | AppMode::SceneConfigEditor | AppMode::EnvConfigEditor => {
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
                                AppMode::EnvConfigEditor => { self.env_config_editor_tool.render_main(ui); }
                                _ => {},
                            }
                        });
                    },
                    AppMode::Battery | AppMode::FarmEntityPlanEditor | AppMode::MovementConfigEditor | AppMode::AgentConfigEditor | AppMode::PerformanceMatrixTool => {
                        // Tools without camera
                        match self.mode {
                        AppMode::Battery => { self.battery_tool.render_main(ui); }
                        AppMode::FarmEntityPlanEditor => self.farm_entity_plan_editor_tool.render_main(ui),
                        AppMode::MovementConfigEditor => self.movement_config_editor_tool.render_main(ui),
                        AppMode::AgentConfigEditor => self.agent_config_editor_tool.render_main(ui),
                        AppMode::PerformanceMatrixTool => self.performance_matrix_tool.render_main(ui),
                        _ => {}
                    }
                },
            }
        });
    }
}
