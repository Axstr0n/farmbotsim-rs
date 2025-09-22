use std::{collections::HashMap, fs};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::tool_module::tool::Tool;

/// Markdown section encapsulating its level, title, content, children
#[derive(Debug, Clone)]
struct MarkdownSection {
    level: usize,
    title: String,
    content: String,
    children: Vec<MarkdownSection>,
}

/// A tool for help
pub struct GeneralHelpTool {
    sections: Vec<MarkdownSection>,
    collapsed_state: HashMap<String, bool>,
    file_path: String,
    raw_content: String,
    commonmark_cache: CommonMarkCache,
}

impl Default for GeneralHelpTool {
    fn default() -> Self {
        let mut tool = Self {
            sections: Vec::new(),
            collapsed_state: HashMap::new(),
            file_path: "general_help/help.md".to_string(),
            raw_content: String::new(),
            commonmark_cache: CommonMarkCache::default(),
        };
        tool.load_markdown();
        tool
    }
}


impl Tool for GeneralHelpTool {
    fn render_main(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if !self.sections.is_empty() {
                    let section_count = self.sections.len();
                    for i in 0..section_count {
                        let section = &self.sections[i].clone();
                        let path = i.to_string();
                        self.render_section(ui, section, &path, 0);
                        ui.add_space(15.0);
                    }

                }
            });
    }
    fn render_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Table of Contents");
        let section_count = self.sections.len();
        for i in 0..section_count {
            let section = &self.sections[i].clone();
            Self::render_toc(ui, section, &i.to_string(), 0);
        }
        ui.separator();
    }
    fn update(&mut self) {
        
    }
}

impl GeneralHelpTool {
    /// Recursively renders table of content.
    fn render_toc(ui: &mut egui::Ui, section: &MarkdownSection, path: &str, indent_level: usize) {
        let indent = indent_level as f32 * 16.0;

        ui.horizontal(|ui| {
            ui.add_space(indent);
            let path = if path.len() == 1 {
                ""
            } else if let Some(path) = path.strip_prefix("0.") {path}
            else {path};
            ui.label(format!("{} {}", path, &section.title));
        });

        for (i, child) in section.children.iter().enumerate() {
            let child_path = format!("{path}.{i}");
            Self::render_toc(ui, child, &child_path, indent_level + 1);
        }
    }

    /// Loads file content.
    fn load_markdown(&mut self) {
        match fs::read_to_string(&self.file_path) {
            Ok(content) => {
                self.raw_content = content;
                let sections = self.parse_markdown_hierarchical(&self.raw_content);
                // Keep existing collapsed states, but initialize new ones as expanded
                self.initialize_collapsed_states(&sections, "");
                self.sections = sections;
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", self.file_path, e);
                self.raw_content.clear();
                self.sections.clear();
            }
        }
    }

    /// Recursively sets collapsed state hashmap.
    fn initialize_collapsed_states(&mut self, sections: &[MarkdownSection], parent_path: &str) {
        for (i, section) in sections.iter().enumerate() {
            let section_path = if parent_path.is_empty() {
                i.to_string()
            } else {
                format!("{parent_path}.{i}")
            };
            
            // Only initialize if not already present (preserves user's collapse state)
            if !self.collapsed_state.contains_key(&section_path) {
                self.collapsed_state.insert(section_path.clone(), false);
            }
            
            // Recursively initialize children
            self.initialize_collapsed_states(&section.children, &section_path);
        }
    }

    /// Parses the whole markdown content into a hierarchical tree of sections.
    fn parse_markdown_hierarchical(&self, content: &str) -> Vec<MarkdownSection> {
        let flat_sections = self.parse_markdown_flat(content);
        self.build_hierarchy(flat_sections)
    }

    /// Parses markdown content into a flat list of sections with levels and titles.
    fn parse_markdown_flat(&self, content: &str) -> Vec<MarkdownSection> {
        let lines: Vec<&str> = content.lines().collect();
        let mut sections = Vec::new();
        let mut current_section: Option<MarkdownSection> = None;
        let mut current_content_lines = Vec::new();

        for line in lines.iter() {
            if line.starts_with('#') {
                // Save previous section if exists
                if let Some(mut section) = current_section.take() {
                    section.content = current_content_lines.join("\n");
                    sections.push(section);
                    current_content_lines.clear();
                }

                // Parse header level and title
                let level = line.chars().take_while(|&c| c == '#').count();
                let title = line.trim_start_matches('#').trim().to_string();

                current_section = Some(MarkdownSection {
                    level,
                    title,
                    content: String::new(),
                    children: Vec::new(),
                });
            } else if current_section.is_some() {
                // Add content to current section
                current_content_lines.push(*line);
            }
        }

        // Don't forget the last section
        if let Some(mut section) = current_section {
            section.content = current_content_lines.join("\n");
            sections.push(section);
        }

        sections
    }

    /// Builds a tree hierarchy of sections from a flat list based on header levels.
    fn build_hierarchy(&self, flat_sections: Vec<MarkdownSection>) -> Vec<MarkdownSection> {
        let mut root_sections = Vec::new();
        let mut stack: Vec<MarkdownSection> = Vec::new();

        for section in flat_sections {
        while let Some(top) = stack.last() {
            if top.level >= section.level {
                // Instead of unwrap, use if let Some after pop
                if let Some(popped) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(popped);
                    } else {
                        root_sections.push(popped);
                    }
                }
            } else {
                break;
            }
        }
        stack.push(section);
    }


        // Pop remaining sections from stack
        while let Some(section) = stack.pop() {
            if let Some(parent) = stack.last_mut() {
                parent.children.push(section);
            } else {
                root_sections.push(section);
            }
        }

        root_sections
    }

    /// Recursively renders a markdown section with indentation and collapsible headers.
    fn render_section(&mut self, ui: &mut egui::Ui, section: &MarkdownSection, path: &str, indent_level: usize) {
        let is_collapsed = *self.collapsed_state.get(path).unwrap_or(&false);

        // Add indentation for nested sections
        let indent = indent_level as f32 * 20.0;
        
        ui.horizontal(|ui| {
            // Add indentation spacing
            if indent > 0.0 {
                ui.add_space(indent);
            }

            // Create collapsing header with level-appropriate styling
            let header_text = match section.level {
                1 => egui::RichText::new(&section.title).size(24.0).strong(),
                2 => egui::RichText::new(&section.title).size(20.0).strong(),
                3 => egui::RichText::new(&section.title).size(18.0).strong(),
                4 => egui::RichText::new(&section.title).size(16.0).strong(),
                5 => egui::RichText::new(&section.title).size(14.0).strong(),
                _ => egui::RichText::new(&section.title).size(12.0).strong(),
            };

            let header_response = egui::CollapsingHeader::new(header_text)
                .default_open(!is_collapsed)
                .show(ui, |ui| {
                    // Render section content
                    if !section.content.trim().is_empty() {
                        // Add indentation for content
                        ui.horizontal(|ui| {
                            if indent_level > 0 {
                                ui.add_space(20.0); // Additional indent for content
                            }
                            ui.vertical(|ui| {
                                CommonMarkViewer::new()
                                    .show(ui, &mut self.commonmark_cache, &section.content);
                            });
                        });
                    }

                    // Render child sections
                    for (i, child) in section.children.iter().enumerate() {
                        let child_path = format!("{path}.{i}");
                        self.render_section(ui, child, &child_path, indent_level + 1);
                        ui.add_space(10.0);
                    }
                });

            // Update collapsed state based on header click
            if header_response.header_response.clicked() {
                let current_state = *self.collapsed_state.get(path).unwrap_or(&false);
                self.collapsed_state.insert(path.to_string(), !current_state);
            }
        });
    }
}