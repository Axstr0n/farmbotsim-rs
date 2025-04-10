use egui::{Pos2, Vec2};

use super::{crop::Crop, field_config::FieldConfig, obstacle::Obstacle};

use crate::utilities::vec2::Vec2Rotate;


#[derive(PartialEq, Clone)]
pub struct Field {
    pub config: FieldConfig,
    pub crops: Vec<Crop>,
    pub obstacles: Vec<Obstacle>,
}

impl Field {
    pub fn from_config(config: Option<FieldConfig>) -> Self {
        let config = config.unwrap_or_default();
        let crops = Self::calculate_crops(&config);
        let obstacles = Self::calculate_obstacles(&config);
        Self {
            config,
            crops,
            obstacles,
        }
    }

    pub fn calculate_crops(field_config: &FieldConfig) -> Vec<Crop>{
        let mut crops: Vec<Crop> = Vec::new();

        let left_top_pos = field_config.left_top_pos;
        let angle = field_config.angle;
        let n_rows = field_config.n_rows;
        let row_spacing = field_config.row_spacing;
        let n_crops_per_row = field_config.n_crops_per_row;
        let crop_spacing = field_config.crop_spacing;
        // let row_length = (n_crops_per_row-1) as f32 * crop_spacing;
        // let field_length = (n_rows-1) as f32 * row_spacing;

        let top_pos = left_top_pos;

        for i in 0..n_rows {
            for j in 0..n_crops_per_row {
                let pos = top_pos + Vec2::new(row_spacing*i as f32, crop_spacing*j as f32).rotate_degrees(angle);
                crops.push(
                    Crop::new(j*i, i, pos)
                );
            }
        }

        crops
    }

    pub fn calculate_obstacles(field_config: &FieldConfig) -> Vec<Obstacle> {
        let mut obstacles: Vec<Obstacle> = Vec::new();

        let left_top_pos = field_config.left_top_pos;
        let angle = field_config.angle;
        let n_rows = field_config.n_rows;
        let row_spacing = field_config.row_spacing;
        let n_crops_per_row = field_config.n_crops_per_row;
        let crop_spacing = field_config.crop_spacing;
        let row_length = (n_crops_per_row-1) as f32 * crop_spacing;
        // let field_length = (n_rows-1) as f32 * row_spacing;

        let obstacle_width = 0.08;
        let height_offset = 0.2;
        let mut pos1 = left_top_pos + Vec2::new(-row_spacing/2.0, -height_offset).rotate_degrees(angle);
        for _ in 0..n_rows+1 {
            let pos2 = pos1 + Vec2::new(row_length+2.0*height_offset, 0.0).rotate_degrees(angle+90.0);
            let p1 = pos1 + Vec2::new(-obstacle_width/2.0,0.0).rotate_degrees(angle);
            let p2 = pos1 + Vec2::new(obstacle_width/2.0,0.0).rotate_degrees(angle);
            let p3 = pos2 + Vec2::new(obstacle_width/2.0,0.0).rotate_degrees(angle);
            let p4 = pos2 + Vec2::new(-obstacle_width/2.0,0.0).rotate_degrees(angle);
            obstacles.push(Obstacle::new(vec![p1,p2,p3,p4]));
            pos1 += Vec2::new(row_spacing, 0.0).rotate_degrees(angle);
        }

        obstacles
    }

    pub fn get_graph_points(&self) -> Vec<Pos2> {
        let mut points = Vec::new();

        let left_top_pos = self.config.left_top_pos;
        let angle = self.config.angle;
        let n_rows = self.config.n_rows;
        let row_spacing = self.config.row_spacing;
        let n_crops_per_row = self.config.n_crops_per_row;
        let crop_spacing = self.config.crop_spacing;
        let row_length = (n_crops_per_row-1) as f32 * crop_spacing;
        // let field_length = (n_rows-1) as f32 * row_spacing;

        let top_pos = left_top_pos;
        let offset = 0.5;

        for i in (-1)..=n_rows as i32 {
            let p1 = top_pos + Vec2::new(row_spacing*i as f32, -offset as f32).rotate_degrees(angle);
            let p2 = top_pos + Vec2::new(row_spacing*i as f32, row_length+offset as f32).rotate_degrees(angle);
            points.push(p1);
            points.push(p2);
        }

        points
    }

    pub fn recalculate_field(&mut self) {
        self.crops = Self::calculate_crops(&self.config);
        self.obstacles = Self::calculate_obstacles(&self.config);
    }

}


