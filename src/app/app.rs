use eframe::egui;
use std::time::{Duration, Instant};


use super::app_mode::AppMode;
use crate::tool::tool::Tool;
use crate::tool::simulation_tool::SimulationTool;
use crate::tool::path_tool::PathTool;
use crate::tool::editor_tool::EditorTool;

pub struct App {
    mode: AppMode,
    is_dark_mode: bool,

    simulation_tool: SimulationTool,
    path_tool: PathTool,
    editor_tool: EditorTool,

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
            mode: AppMode::Editor,
            is_dark_mode: true,

            simulation_tool: SimulationTool::default(),
            path_tool: PathTool::default(),
            editor_tool: EditorTool::default(),

            fps: 0.0,
            tps: 0.0,
            ratio_tps_fps: 1.0,

            ticks: 0,
            frames: 0,
            frame_count: 0,
            tick_count: 0,
            last_stat_time: Instant::now(),

            accumulator: 0.0,
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

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
            AppMode::Editor => self.editor_tool.update(),
            AppMode::ConfigEditor => {},
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
                        egui::Slider::new(&mut self.ratio_tps_fps, 0.01..=100.0)
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
            .default_width(200.0)
            .min_width(200.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                
                ui.heading(self.mode.to_string());
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    
                    match self.mode {
                        AppMode::Simulation => self.simulation_tool.render_ui(ui),
                        AppMode::Path => self.path_tool.render_ui(ui),
                        AppMode::Editor => self.editor_tool.render_ui(ui),//self.editor_tool.render_ui(ui),
                        AppMode::ConfigEditor => {},
                    }
                });
        });
        
        // Main scene area for (zoom/pan) or text editor
        egui::CentralPanel::default().show(ctx, |ui| {

            match self.mode {
                AppMode::Editor | AppMode::Simulation | AppMode::Path => {
                    // Tools with camera
                    egui::Frame::group(ui.style())
                        .inner_margin(0.0)
                        .show(ui, |ui| {
                            match self.mode {
                                AppMode::Editor => { self.editor_tool.render_main(ui); }
                                AppMode::Simulation => { self.simulation_tool.render_main(ui); }
                                AppMode::Path => { self.path_tool.render_main(ui); }
                                AppMode::ConfigEditor => {}
                            }
                        });
                },
                AppMode::ConfigEditor => {
                    // Tools with text editor
                }
            }
        });
    }
}
