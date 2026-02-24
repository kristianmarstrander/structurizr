//! Style definitions for elements and relationships

use std::collections::HashMap;

/// Shape types for elements (from Shape.java)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    Box,
    RoundedBox,
    Circle,
    Ellipse,
    Hexagon,
    Diamond,
    Cylinder,
    Pipe,
    Person,
    Robot,
    Folder,
    WebBrowser,
    Window,
    Terminal,
    Shell,
    MobileDevicePortrait,
    MobileDeviceLandscape,
    Component,
}

impl Shape {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "box" => Some(Shape::Box),
            "roundedbox" => Some(Shape::RoundedBox),
            "circle" => Some(Shape::Circle),
            "ellipse" => Some(Shape::Ellipse),
            "hexagon" => Some(Shape::Hexagon),
            "diamond" => Some(Shape::Diamond),
            "cylinder" => Some(Shape::Cylinder),
            "pipe" => Some(Shape::Pipe),
            "person" => Some(Shape::Person),
            "robot" => Some(Shape::Robot),
            "folder" => Some(Shape::Folder),
            "webbrowser" => Some(Shape::WebBrowser),
            "window" => Some(Shape::Window),
            "terminal" => Some(Shape::Terminal),
            "shell" => Some(Shape::Shell),
            "mobiledeviceportrait" => Some(Shape::MobileDevicePortrait),
            "mobiledevicelandscape" => Some(Shape::MobileDeviceLandscape),
            "component" => Some(Shape::Component),
            _ => None,
        }
    }
}

/// Border styles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Border {
    Solid,
    Dashed,
    Dotted,
}

impl Border {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "solid" => Some(Border::Solid),
            "dashed" => Some(Border::Dashed),
            "dotted" => Some(Border::Dotted),
            _ => None,
        }
    }

    pub fn to_dasharray(&self) -> &str {
        match self {
            Border::Solid => "",
            Border::Dashed => "5,5",
            Border::Dotted => "2,2",
        }
    }
}

/// Element style definition
#[derive(Debug, Clone)]
pub struct ElementStyle {
    pub tag: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub background: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<u32>,
    pub color: Option<String>,
    pub font_size: Option<u32>,
    pub shape: Option<Shape>,
    pub border: Option<Border>,
    pub opacity: Option<u32>,
    pub metadata: Option<bool>,
    pub description: Option<bool>,
}

impl ElementStyle {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            width: None,
            height: None,
            background: None,
            stroke: None,
            stroke_width: None,
            color: None,
            font_size: None,
            shape: None,
            border: None,
            opacity: None,
            metadata: None,
            description: None,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width.unwrap_or(450)
    }

    pub fn get_height(&self) -> u32 {
        self.height.unwrap_or(300)
    }

    pub fn get_background(&self) -> String {
        self.background.clone().unwrap_or_else(|| "#438dd5".to_string())
    }

    pub fn get_stroke(&self) -> String {
        self.stroke.clone().unwrap_or_else(|| self.get_background())
    }

    pub fn get_stroke_width(&self) -> u32 {
        self.stroke_width.unwrap_or(2)
    }

    pub fn get_color(&self) -> String {
        self.color.clone().unwrap_or_else(|| "#ffffff".to_string())
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size.unwrap_or(24)
    }

    pub fn get_shape(&self) -> Shape {
        self.shape.clone().unwrap_or(Shape::RoundedBox)
    }

    pub fn get_opacity(&self) -> u32 {
        self.opacity.unwrap_or(100)
    }

    pub fn get_dasharray(&self) -> &str {
        self.border.as_ref().map_or("", |b| b.to_dasharray())
    }
}

/// Relationship routing styles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Routing {
    Direct,
    Orthogonal,
    Curved,
}

/// Relationship style definition
#[derive(Debug, Clone)]
pub struct RelationshipStyle {
    pub tag: String,
    pub color: Option<String>,
    pub thickness: Option<u32>,
    pub style: Option<Border>,
    pub routing: Option<Routing>,
    pub font_size: Option<u32>,
    pub position: Option<u32>,
}

impl RelationshipStyle {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            color: None,
            thickness: None,
            style: None,
            routing: None,
            font_size: None,
            position: None,
        }
    }

    pub fn get_color(&self) -> String {
        self.color.clone().unwrap_or_else(|| "#707070".to_string())
    }

    pub fn get_thickness(&self) -> u32 {
        self.thickness.unwrap_or(2)
    }

    pub fn get_dasharray(&self) -> &str {
        self.style.as_ref().map_or("", |s| s.to_dasharray())
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size.unwrap_or(24)
    }
}

/// Collection of styles
#[derive(Debug, Clone, Default)]
pub struct Styles {
    pub element_styles: HashMap<String, ElementStyle>,
    pub relationship_styles: HashMap<String, RelationshipStyle>,
}

impl Styles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_element_style(&mut self, style: ElementStyle) {
        self.element_styles.insert(style.tag.clone(), style);
    }

    pub fn add_relationship_style(&mut self, style: RelationshipStyle) {
        self.relationship_styles.insert(style.tag.clone(), style);
    }

    pub fn get_element_style(&self, tags: &[String]) -> ElementStyle {
        // Find the most specific style that matches any tag
        // Start with default style
        let mut result = ElementStyle::new("Element".to_string());
        
        // Apply styles in order of specificity
        for tag in tags {
            if let Some(style) = self.element_styles.get(tag) {
                // Merge styles
                if style.width.is_some() {
                    result.width = style.width;
                }
                if style.height.is_some() {
                    result.height = style.height;
                }
                if style.background.is_some() {
                    result.background = style.background.clone();
                }
                if style.stroke.is_some() {
                    result.stroke = style.stroke.clone();
                }
                if style.stroke_width.is_some() {
                    result.stroke_width = style.stroke_width;
                }
                if style.color.is_some() {
                    result.color = style.color.clone();
                }
                if style.font_size.is_some() {
                    result.font_size = style.font_size;
                }
                if style.shape.is_some() {
                    result.shape = style.shape.clone();
                }
                if style.border.is_some() {
                    result.border = style.border.clone();
                }
                if style.opacity.is_some() {
                    result.opacity = style.opacity;
                }
            }
        }
        
        result
    }

    pub fn get_relationship_style(&self, tags: &[String]) -> RelationshipStyle {
        let mut result = RelationshipStyle::new("Relationship".to_string());
        
        for tag in tags {
            if let Some(style) = self.relationship_styles.get(tag) {
                if style.color.is_some() {
                    result.color = style.color.clone();
                }
                if style.thickness.is_some() {
                    result.thickness = style.thickness;
                }
                if style.style.is_some() {
                    result.style = style.style.clone();
                }
                if style.routing.is_some() {
                    result.routing = style.routing.clone();
                }
                if style.font_size.is_some() {
                    result.font_size = style.font_size;
                }
            }
        }
        
        result
    }
}
