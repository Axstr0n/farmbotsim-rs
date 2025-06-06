use std::collections::HashMap;
use egui::{Pos2, Vec2};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    cfg::{DEFAULT_LINE_FARM_ENTITY_PLAN_PATH, DEFAULT_POINT_FARM_ENTITY_PLAN_PATH}, environment::{
        farm_entity_module::{
            crop::Crop,
            farm_entity::FarmEntity,
            farm_entity_plan::FarmEntityPlan,
            row::Row,
        },
        obstacle::Obstacle,
    }, units::{
        angle::Angle,
        length::Length,
    }, utilities::vec2::Vec2Rotate
};


#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LineFieldConfig {
    #[serde(skip)]
    pub id: u32,
    #[serde(skip)]
    pub color: egui::Color32,
    pub left_top_pos: Pos2,
    pub angle: Angle,
    pub n_lines: u32,
    pub length: Length,
    pub line_spacing: Length,
    pub farm_entity_plan_path: String,
}
impl Default for LineFieldConfig {
    fn default() -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos: Pos2::new(2.0, 2.0),
            angle: Angle::degrees(0.0),
            n_lines: 3,
            length: Length::meters(4.0),
            line_spacing: Length::meters(0.4),
            farm_entity_plan_path: DEFAULT_LINE_FARM_ENTITY_PLAN_PATH.to_string(),
        }
    }
}
impl LineFieldConfig {
    pub fn new(left_top_pos: Pos2, angle: Angle, n_lines: u32, length: Length, line_spacing: Length, farm_entity_plan_path: String) -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos,
            angle,
            n_lines,
            length,
            line_spacing,
            farm_entity_plan_path,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PointFieldConfig {
    #[serde(skip)]
    pub id: u32,
    #[serde(skip)]
    pub color: egui::Color32,
    pub left_top_pos: Pos2,
    pub angle: Angle,
    pub n_lines: u32,
    pub n_points_per_line: u32,
    pub line_spacing: Length,
    pub point_spacing: Length,
    pub farm_entity_plan_path: String,
}
impl Default for PointFieldConfig {
    fn default() -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos: Pos2::new(2.0, 2.0),
            angle: Angle::degrees(0.0),
            n_lines: 3,
            n_points_per_line: 6,
            line_spacing: Length::meters(0.4),
            point_spacing: Length::meters(0.3),
            farm_entity_plan_path: DEFAULT_POINT_FARM_ENTITY_PLAN_PATH.to_string(),
        }
    }
}
impl PointFieldConfig {
    pub fn new(left_top_pos: Pos2, angle: Angle, n_lines: u32, n_points_per_line: u32, line_spacing: Length, point_spacing: Length, farm_entity_plan_path: String) -> Self {
        Self {
            id: 0,
            color: egui::Color32::WHITE,
            left_top_pos,
            angle,
            n_lines,
            n_points_per_line,
            line_spacing,
            point_spacing,
            farm_entity_plan_path,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum VariantFieldConfig {
    Line(LineFieldConfig),
    Point(PointFieldConfig),
}

impl Serialize for VariantFieldConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VariantFieldConfig::Line(config) => config.serialize(serializer),
            VariantFieldConfig::Point(config) => config.serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for VariantFieldConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First deserialize into a generic Value
        let value = serde_json::Value::deserialize(deserializer)?;
        
        // Try to deserialize as LineFieldConfig
        if let Ok(config) = LineFieldConfig::deserialize(&value) {
            return Ok(VariantFieldConfig::Line(config));
        }
        
        // Try to deserialize as PointFieldConfig
        if let Ok(config) = PointFieldConfig::deserialize(&value) {
            return Ok(VariantFieldConfig::Point(config));
        }
        
        Err(de::Error::custom(format!(
            "Could not determine config type for variant field config from JSON: {}",
            value
        )))
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct FieldConfig {
    pub configs: Vec<VariantFieldConfig>,
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
            let obstacle_width = Length::meters(0.08);
            let height_offset = Length::meters(0.2);
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    let mut pos1 = c.left_top_pos + Vec2::new((-c.line_spacing/2.0).to_base_unit(), (-height_offset).to_base_unit()).rotate(c.angle);
                    let obstacle_width_2_val = (obstacle_width/2.0).to_base_unit();
                    for _ in 0..c.n_lines+1 {
                        let pos2 = pos1 + Vec2::new((c.length+2.0*height_offset).to_base_unit(), 0.0).rotate(c.angle+Angle::degrees(90.0));
                        let p1 = pos1 + Vec2::new(-obstacle_width_2_val,0.0).rotate(c.angle);
                        let p2 = pos1 + Vec2::new(obstacle_width_2_val,0.0).rotate(c.angle);
                        let p3 = pos2 + Vec2::new(obstacle_width_2_val,0.0).rotate(c.angle);
                        let p4 = pos2 + Vec2::new(-obstacle_width_2_val,0.0).rotate(c.angle);
                        obstacles.push(Obstacle::new(vec![p1,p2,p3,p4]));
                        pos1 += Vec2::new(c.line_spacing.to_base_unit(), 0.0).rotate(c.angle);
                    }
                },
                VariantFieldConfig::Point(c) => {
                    let line_length = (c.n_points_per_line-1) as f32 * c.point_spacing;
            
                    let mut pos1 = c.left_top_pos + Vec2::new((-c.line_spacing/2.0).to_base_unit(), (-height_offset).to_base_unit()).rotate(c.angle);
                    let obstacle_width_2_val = (obstacle_width/2.0).to_base_unit();
                    for _ in 0..c.n_lines+1 {
                        let pos2 = pos1 + Vec2::new((line_length+2.0*height_offset).to_base_unit(), 0.0).rotate(c.angle+Angle::degrees(90.0));
                        let p1 = pos1 + Vec2::new(-obstacle_width_2_val,0.0).rotate(c.angle);
                        let p2 = pos1 + Vec2::new(obstacle_width_2_val,0.0).rotate(c.angle);
                        let p3 = pos2 + Vec2::new(obstacle_width_2_val,0.0).rotate(c.angle);
                        let p4 = pos2 + Vec2::new(-obstacle_width_2_val,0.0).rotate(c.angle);
                        obstacles.push(Obstacle::new(vec![p1,p2,p3,p4]));
                        pos1 += Vec2::new(c.line_spacing.to_base_unit(), 0.0).rotate(c.angle);
                    }
                }
            }
        }
        obstacles
    }

    pub fn get_graph_points(&self) -> Vec<Pos2> {
        let mut points = Vec::new();

        for config_variant in &self.configs {
            let offset = Length::meters(0.5);
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    let ls_val = c.line_spacing.to_base_unit();
                    for i in (-1)..=c.n_lines as i32 {
                        let p1 = c.left_top_pos + Vec2::new(ls_val*i as f32, (-offset).to_base_unit()).rotate(c.angle);
                        let p2 = c.left_top_pos + Vec2::new(ls_val*i as f32, c.length.to_base_unit()+offset.to_base_unit()).rotate(c.angle);
                        points.push(p1);
                        points.push(p2);
                    }
                },
                VariantFieldConfig::Point(c) => {
                    let line_length = (c.n_points_per_line-1) as f32 * c.point_spacing;
                    let ls_val = c.line_spacing.to_base_unit();
                    for i in (-1)..=c.n_lines as i32 {
                        let p1 = c.left_top_pos + Vec2::new(ls_val*i as f32, (-offset).to_base_unit()).rotate(c.angle);
                        let p2 = c.left_top_pos + Vec2::new(ls_val*i as f32, line_length.to_base_unit()+offset.to_base_unit()).rotate(c.angle);
                        points.push(p1);
                        points.push(p2);
                    }
                }
            }
        }
        points
    }
    
    pub fn get_farm_entities(&self) -> HashMap<u32, FarmEntity> {
        let mut farm_entities = HashMap::new();

        let mut id_counter = 0;

        for (n, config_variant) in self.configs.iter().enumerate() {
            let field_id = n as u32;
            match config_variant {
                VariantFieldConfig::Line(c) => {
                    let farm_entity_plan = FarmEntityPlan::from_json_file(&c.farm_entity_plan_path);
                    let ls_val = c.line_spacing.value;
                    for i in 0..c.n_lines {
                        let path = vec![c.left_top_pos+Vec2::new(i as f32*ls_val, 0.0).rotate(c.angle), c.left_top_pos+Vec2::new(i as f32*ls_val, c.length.to_base_unit()).rotate(c.angle)];
                        let row = Row::new(id_counter, field_id, path, farm_entity_plan.clone());
                        farm_entities.insert(id_counter, FarmEntity::Row(row));
                        id_counter += 1;
                    }
                },
                VariantFieldConfig::Point(c) => {
                    let farm_entity_plan = FarmEntityPlan::from_json_file(&c.farm_entity_plan_path);
                    for i in 0..c.n_lines {
                        for j in 0..c.n_points_per_line {
                            let pos = c.left_top_pos + Vec2::new(c.line_spacing.to_base_unit()*i as f32, c.point_spacing.to_base_unit()*j as f32).rotate(c.angle);
                            let crop = Crop::new(id_counter, field_id, i, pos, farm_entity_plan.clone());
                            farm_entities.insert(id_counter, FarmEntity::Crop(crop));
                            id_counter += 1;
                        }
                    }
                }
            }
        }

        farm_entities
    }

}