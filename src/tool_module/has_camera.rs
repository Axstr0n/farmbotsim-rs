use crate::rendering::camera::Camera;
use crate::utilities::pos2::ExtendedPos2;

pub trait HasCamera {
    fn camera(&self) -> &Camera;
    fn ui_mouse_position(&self, ui: &mut egui::Ui) {
        let (mouse_pos, scene_pos) = match self.camera().mouse_position {
            Some(pos) => {
                let scene_pos = self.camera().screen_to_scene_pos(pos);
                (Some(pos), Some(scene_pos))
            },
            None => {
                (None, None)
            },
        };
        ui.label(egui::RichText::new("Mouse position:").size(16.0));
        ui.label(format!("Screen pos: {}", mouse_pos.map_or("None".to_string(), |p| p.fmt(2))));
        ui.label(format!("Scene pos: {}", scene_pos.map_or("None".to_string(), |p| p.fmt(2))));
    }
}

macro_rules! impl_has_camera {
    ($t:ty) => {
        impl HasCamera for $t {
            fn camera(&self) -> &Camera {
                &self.camera
            }
        }
    };
}

impl_has_camera!(super::simulation_tool::SimulationTool);
impl_has_camera!(super::path_tool::PathTool);
impl_has_camera!(super::task_tool::TaskTool);
impl_has_camera!(super::field_config_editor_tool::FieldConfigEditorTool);
impl_has_camera!(super::scene_config_editor_tool::SceneConfigEditorTool);
impl_has_camera!(super::env_config_editor_tool::EnvConfigEditorTool);