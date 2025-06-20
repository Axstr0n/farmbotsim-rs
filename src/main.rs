#![deny(clippy::unwrap_used)]       // Disallow unwrap
#![deny(clippy::expect_used)]       // Disallow expect
#![deny(clippy::clone_on_copy)]     // Disallow cloning unnecessarily
#![deny(clippy::redundant_clone)]   // Disallow redundant clones
#![deny(clippy::manual_map)]        // Disallow manual map
#![deny(clippy::manual_filter)]     // Disallow manual filter
// #![deny(clippy::panic)]             // Disallow panic!
#![deny(clippy::borrowed_box)]      // Disallow unnecessary borrowed Box
#![deny(clippy::dbg_macro)]         // Disallow dbg!() in production code
#![deny(clippy::vec_init_then_push)] // Disallow inefficient Vec initialization
#![deny(clippy::cast_lossless)]     // Disallow using `as` for conversions that could fail


pub mod app_module;
use crate::app_module::app::App;

pub mod tool_module;
pub mod rendering;
pub mod agent_module;
pub mod movement_module;
pub mod environment;
pub mod path_finding_module;
pub mod task_module;
pub mod utilities;
pub mod units;
pub mod cfg;
pub mod logger;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        vsync: true,
        viewport: egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_maximized(true)
            // .with_inner_size([1200.0, 800.0]) // Width, height
            // .with_min_inner_size([400.0, 300.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "farmbotsim-rs",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}