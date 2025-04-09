mod app;
use crate::app::app::App;

mod tool;
mod rendering;
mod agent;
mod environment;
mod path;
mod utilities;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        vsync: true,
        ..Default::default()
    };
    eframe::run_native(
        "farmbotsim-rs",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
