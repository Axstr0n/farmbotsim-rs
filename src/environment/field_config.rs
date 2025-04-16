use egui::{Pos2, Vec2};

use crate::utilities::vec2::Vec2Rotate;
use super::obstacle::Obstacle;


#[derive(PartialEq, Clone, Copy)]
pub struct LineFieldConfig {
    pub id: u32,
    pub color: egui::Color32,
    pub left_top_pos: Pos2,
    pub angle: f32,
    pub n_lines: u32,
    pub length: f32,
    pub line_spacing: f32,
}
impl Default for LineFieldConfig {
    fn default() -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos: Pos2::new(2.0, 2.0),
            angle: 0.0,
            n_lines: 3,
            length: 4.0,
            line_spacing: 0.4,
        }
    }
}
impl LineFieldConfig {
    pub fn new(left_top_pos: Pos2, angle: f32, n_lines: u32, length: f32, line_spacing: f32) -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos,
            angle,
            n_lines,
            length,
            line_spacing,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct PointFieldConfig {
    pub id: u32,
    pub color: egui::Color32,
    pub left_top_pos: Pos2,
    pub angle: f32,
    pub n_lines: u32,
    pub n_points_per_line: u32,
    pub line_spacing: f32,
    pub point_spacing: f32,
}
impl Default for PointFieldConfig {
    fn default() -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos: Pos2::new(2.0, 2.0),
            angle: 0.0,
            n_lines: 3,
            n_points_per_line: 6,
            line_spacing: 0.4,
            point_spacing: 0.3,
        }
    }
}
impl PointFieldConfig {
    pub fn new(left_top_pos: Pos2, angle: f32, n_lines: u32, n_points_per_line: u32, line_spacing: f32, point_spacing: f32) -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos,
            angle,
            n_lines,
            n_points_per_line,
            line_spacing,
            point_spacing,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum VariantFieldConfig {
    Line(LineFieldConfig),
    Point(PointFieldConfig),
}


#[derive(PartialEq, Clone)]
pub struct FieldConfig {
    pub configs: Vec<VariantFieldConfig>,
}

impl Default for FieldConfig {
    fn default() -> Self {
        Self {
            configs: vec![
                VariantFieldConfig::Line(LineFieldConfig::new(Pos2::new(3.0, 4.0), 0.0, 3, 5.0, 0.5)),
                VariantFieldConfig::Line(LineFieldConfig::new(Pos2::new(12.0, 4.0), 0.0, 3, 5.0, 0.5)),
                VariantFieldConfig::Point(PointFieldConfig::new(Pos2::new(7.0, 4.0), 0.0, 5, 8, 0.3, 0.3)),
                VariantFieldConfig::Point(PointFieldConfig::new(Pos2::new(15.0, 4.0), 0.0, 5, 8, 0.3, 0.3)),
            ]
        }
    }
}

impl FieldConfig {
    pub fn new(configs: Vec<VariantFieldConfig>) -> Self {
        Self {
            configs
        }
    }

    pub fn get_obstacles(&self) -> Vec<Obstacle> {
        let mut obstacles: Vec<Obstacle> = Vec::new();

        for config_variant in &self.configs {
            let obstacle_width = 0.08;
            let height_offset = 0.2;
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    let mut pos1 = c.left_top_pos + Vec2::new(-c.line_spacing/2.0, -height_offset).rotate_degrees(c.angle);
                    for _ in 0..c.n_lines+1 {
                        let pos2 = pos1 + Vec2::new(c.length+2.0*height_offset, 0.0).rotate_degrees(c.angle+90.0);
                        let p1 = pos1 + Vec2::new(-obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        let p2 = pos1 + Vec2::new(obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        let p3 = pos2 + Vec2::new(obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        let p4 = pos2 + Vec2::new(-obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        obstacles.push(Obstacle::new(vec![p1,p2,p3,p4]));
                        pos1 += Vec2::new(c.line_spacing, 0.0).rotate_degrees(c.angle);
                    }
                },
                VariantFieldConfig::Point(c) => {
                    let line_length = (c.n_points_per_line-1) as f32 * c.point_spacing;
            
                    let mut pos1 = c.left_top_pos + Vec2::new(-c.line_spacing/2.0, -height_offset).rotate_degrees(c.angle);
                    for _ in 0..c.n_lines+1 {
                        let pos2 = pos1 + Vec2::new(line_length+2.0*height_offset, 0.0).rotate_degrees(c.angle+90.0);
                        let p1 = pos1 + Vec2::new(-obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        let p2 = pos1 + Vec2::new(obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        let p3 = pos2 + Vec2::new(obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        let p4 = pos2 + Vec2::new(-obstacle_width/2.0,0.0).rotate_degrees(c.angle);
                        obstacles.push(Obstacle::new(vec![p1,p2,p3,p4]));
                        pos1 += Vec2::new(c.line_spacing, 0.0).rotate_degrees(c.angle);
                    }
                }
            }
        }
        obstacles
    }

    pub fn get_graph_points(&self) -> Vec<Pos2> {
        let mut points = Vec::new();

        for config_variant in &self.configs {
            let offset = 0.5;
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    for i in (-1)..=c.n_lines as i32 {
                        let p1 = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, -offset as f32).rotate_degrees(c.angle);
                        let p2 = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, c.length+offset as f32).rotate_degrees(c.angle);
                        points.push(p1);
                        points.push(p2);
                    }
                },
                VariantFieldConfig::Point(c) => {
                    let line_length = (c.n_points_per_line-1) as f32 * c.point_spacing;
                    for i in (-1)..=c.n_lines as i32 {
                        let p1 = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, -offset as f32).rotate_degrees(c.angle);
                        let p2 = c.left_top_pos + Vec2::new(c.line_spacing*i as f32, line_length+offset as f32).rotate_degrees(c.angle);
                        points.push(p1);
                        points.push(p2);
                    }
                }
            }
        }
        points
    }
}