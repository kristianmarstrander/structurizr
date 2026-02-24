//! Main SVG renderer

use crate::model::{Direction, View, Workspace};
use crate::renderer::{Layout, ShapeRenderer, TextRenderer};
use anyhow::Result;
use svg::node::element::{Definitions, Group, Line, Marker, Path, Text as SvgText};
use svg::Document;

pub struct SvgRenderer {
    canvas_width: u32,
    canvas_height: u32,
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            canvas_width: 2000,
            canvas_height: 1500,
        }
    }

    pub fn with_dimensions(width: u32, height: u32) -> Self {
        Self {
            canvas_width: width,
            canvas_height: height,
        }
    }

    pub fn render_view(&self, workspace: &Workspace, view: &View) -> Result<String> {
        // Calculate layout if auto-layout is enabled
        let mut layout = Layout::new();
        if let Some(auto_layout) = &view.auto_layout {
            layout.calculate_layout(workspace, view, &auto_layout.direction);
        } else {
            // Simple default layout
            layout.calculate_layout(workspace, view, &Direction::TopBottom);
        }

        // Compute canvas size dynamically from element positions and dimensions
        let padding = 80u32;
        let mut max_x = 0i32;
        let mut max_y = 0i32;
        for element_id in &view.elements {
            if let (Some(pos), Some(element)) = (
                layout.get_position(element_id),
                workspace.get_element(element_id),
            ) {
                let style = workspace.styles.get_element_style(&element.tags);
                max_x = max_x.max(pos.x + style.get_width() as i32);
                max_y = max_y.max(pos.y + style.get_height() as i32);
            }
        }
        let canvas_width = (max_x as u32 + padding).max(self.canvas_width.min(400));
        let canvas_height = (max_y as u32 + padding).max(self.canvas_height.min(400));

        let mut document = Document::new()
            .set("width", canvas_width)
            .set("height", canvas_height)
            .set("viewBox", format!("0 0 {} {}", canvas_width, canvas_height))
            .set("xmlns", "http://www.w3.org/2000/svg");

        // Add definitions (for arrow markers)
        let defs = self.create_definitions();
        document = document.add(defs);

        // Render relationships first (so they appear behind elements)
        for rel_id in &view.relationships {
            if let Some(rel) = workspace.relationships.iter().find(|r| r.id == *rel_id) {
                if let (Some(src_pos), Some(dst_pos)) = (
                    layout.get_position(&rel.source_id),
                    layout.get_position(&rel.destination_id),
                ) {
                    let src_element = workspace.get_element(&rel.source_id);
                    let dst_element = workspace.get_element(&rel.destination_id);

                    if let (Some(src), Some(dst)) = (src_element, dst_element) {
                        let src_style = workspace.styles.get_element_style(&src.tags);
                        let dst_style = workspace.styles.get_element_style(&dst.tags);
                        let rel_style = workspace.styles.get_relationship_style(&rel.tags);

                        let rel_group = self.render_relationship(
                            src_pos.x,
                            src_pos.y,
                            src_style.get_width(),
                            src_style.get_height(),
                            dst_pos.x,
                            dst_pos.y,
                            dst_style.get_width(),
                            dst_style.get_height(),
                            &rel.description,
                            &rel_style.get_color(),
                            rel_style.get_thickness(),
                            rel_style.get_dasharray(),
                        );
                        document = document.add(rel_group);
                    }
                }
            }
        }

        // Render elements
        for element_id in &view.elements {
            if let Some(element) = workspace.get_element(element_id) {
                if let Some(pos) = layout.get_position(element_id) {
                    let style = workspace.styles.get_element_style(&element.tags);
                    
                    let element_group = self.render_element(
                        element,
                        pos.x,
                        pos.y,
                        &style,
                    );
                    document = document.add(element_group);
                }
            }
        }

        // Convert to string
        Ok(document.to_string())
    }

    fn create_definitions(&self) -> Definitions {
        let mut defs = Definitions::new();

        // Arrow marker
        let marker = Marker::new()
            .set("id", "arrow")
            .set("markerWidth", 10)
            .set("markerHeight", 10)
            .set("refX", 9)
            .set("refY", 3)
            .set("orient", "auto")
            .set("markerUnits", "strokeWidth")
            .add(
                Path::new()
                    .set("d", "M0,0 L0,6 L9,3 z")
                    .set("fill", "#707070"),
            );

        defs = defs.add(marker);
        defs
    }

    fn render_element(
        &self,
        element: &crate::model::Element,
        x: i32,
        y: i32,
        style: &crate::model::ElementStyle,
    ) -> Group {
        let mut group = Group::new();

        let width = style.get_width();
        let height = style.get_height();
        let shape = style.get_shape();

        // Render shape
        let shape_node = ShapeRenderer::render_shape(&shape, x, y, width, height, style);
        group = group.add(shape_node);

        // Render text
        let text_group = self.render_element_text(
            &element.name,
            element.description.as_deref(),
            element.technology.as_deref(),
            x,
            y,
            width,
            height,
            style,
        );
        group = group.add(text_group);

        group
    }

    fn render_element_text(
        &self,
        name: &str,
        description: Option<&str>,
        technology: Option<&str>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        style: &crate::model::ElementStyle,
    ) -> Group {
        let mut group = Group::new();

        let color = style.get_color();
        let font_size = style.get_font_size();
        let padding = 20;

        // Wrap text
        let name_lines = TextRenderer::wrap_text(name, width - padding * 2, font_size);
        
        let mut all_lines = Vec::new();
        all_lines.extend(name_lines);

        // Add technology if present
        if let Some(tech) = technology {
            let tech_text = format!("[{}]", tech);
            let tech_lines = TextRenderer::wrap_text(&tech_text, width - padding * 2, font_size - 4);
            all_lines.extend(tech_lines);
        }

        // Add description if present and style allows
        if let Some(desc) = description {
            if style.description.unwrap_or(true) {
                let desc_lines = TextRenderer::wrap_text(desc, width - padding * 2, font_size - 6);
                all_lines.extend(desc_lines);
            }
        }

        // Calculate total text height
        let line_spacing = 10;
        let total_height = TextRenderer::calculate_text_height(all_lines.len(), font_size, line_spacing);

        // Center vertically
        let start_y = y + ((height as i32 - total_height as i32) / 2);

        // Render lines
        let text_x = x + (width / 2) as i32;
        let mut current_y = start_y;

        for (i, line) in all_lines.iter().enumerate() {
            let line_font_size = if i == 0 {
                font_size
            } else {
                font_size - 4
            };

            let mut text = SvgText::new(line.clone())
                .set("x", text_x)
                .set("y", current_y)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "hanging")
                .set("fill", color.clone())
                .set("font-size", format!("{}px", line_font_size))
                .set("font-family", "Arial, Helvetica, sans-serif");

            if i == 0 {
                text = text.set("font-weight", "bold");
            }

            group = group.add(text);

            current_y += line_font_size as i32 + line_spacing as i32;
        }

        group
    }

    fn render_relationship(
        &self,
        src_x: i32,
        src_y: i32,
        src_width: u32,
        src_height: u32,
        dst_x: i32,
        dst_y: i32,
        dst_width: u32,
        dst_height: u32,
        description: &str,
        color: &str,
        thickness: u32,
        dasharray: &str,
    ) -> Group {
        let mut group = Group::new();

        // Calculate center points
        let src_center_x = src_x + (src_width / 2) as i32;
        let src_center_y = src_y + (src_height / 2) as i32;
        let dst_center_x = dst_x + (dst_width / 2) as i32;
        let dst_center_y = dst_y + (dst_height / 2) as i32;

        // Find edge intersection points
        let (line_start_x, line_start_y) = self.find_edge_point(
            src_center_x,
            src_center_y,
            dst_center_x,
            dst_center_y,
            src_width,
            src_height,
        );

        let (line_end_x, line_end_y) = self.find_edge_point(
            dst_center_x,
            dst_center_y,
            src_center_x,
            src_center_y,
            dst_width,
            dst_height,
        );

        // Draw line
        let mut line = Line::new()
            .set("x1", line_start_x)
            .set("y1", line_start_y)
            .set("x2", line_end_x)
            .set("y2", line_end_y)
            .set("stroke", color)
            .set("stroke-width", thickness)
            .set("marker-end", "url(#arrow)");

        if !dasharray.is_empty() {
            line = line.set("stroke-dasharray", dasharray);
        }

        group = group.add(line);

        // Add label at midpoint
        if !description.is_empty() {
            let mid_x = (line_start_x + line_end_x) / 2;
            let mid_y = (line_start_y + line_end_y) / 2;

            let text = SvgText::new(description)
                .set("x", mid_x)
                .set("y", mid_y - 5)
                .set("text-anchor", "middle")
                .set("fill", color)
                .set("font-size", "18px")
                .set("font-family", "Arial, Helvetica, sans-serif");

            group = group.add(text);
        }

        group
    }

    fn find_edge_point(
        &self,
        center_x: i32,
        center_y: i32,
        target_x: i32,
        target_y: i32,
        width: u32,
        height: u32,
    ) -> (i32, i32) {
        // Simple edge point calculation - find intersection with rectangle
        let dx = target_x - center_x;
        let dy = target_y - center_y;

        if dx == 0 && dy == 0 {
            return (center_x, center_y);
        }

        let half_width = (width / 2) as f32;
        let half_height = (height / 2) as f32;

        // Calculate the slope
        let slope = dy as f32 / dx as f32;

        // Check which edge the line intersects
        let edge_x = if dx > 0 { half_width } else { -half_width };
        let edge_y = if dy > 0 { half_height } else { -half_height };

        let intersect_x = edge_x;
        let intersect_y = slope * edge_x;

        if intersect_y.abs() <= half_height {
            // Intersects left or right edge
            (center_x + intersect_x as i32, center_y + intersect_y as i32)
        } else {
            // Intersects top or bottom edge
            let intersect_x = edge_y / slope;
            (center_x + intersect_x as i32, center_y + edge_y as i32)
        }
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}
