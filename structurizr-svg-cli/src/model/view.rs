//! View definitions

/// View types in the Structurizr model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewType {
    SystemLandscape,
    SystemContext,
    Container,
    Component,
    Dynamic,
    Deployment,
}

/// Auto-layout direction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    TopBottom,
    BottomTop,
    LeftRight,
    RightLeft,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "tb" | "topbottom" => Some(Direction::TopBottom),
            "bt" | "bottomtop" => Some(Direction::BottomTop),
            "lr" | "leftright" => Some(Direction::LeftRight),
            "rl" | "rightleft" => Some(Direction::RightLeft),
            _ => None,
        }
    }
}

/// Auto-layout configuration
#[derive(Debug, Clone)]
pub struct AutoLayout {
    pub direction: Direction,
    pub rank_separation: u32,
    pub node_separation: u32,
}

impl AutoLayout {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            rank_separation: 300,
            node_separation: 300,
        }
    }
}

/// A view in the model
#[derive(Debug, Clone)]
pub struct View {
    pub key: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub view_type: ViewType,
    pub elements: Vec<String>, // Element IDs to include
    pub relationships: Vec<String>, // Relationship IDs to include
    pub auto_layout: Option<AutoLayout>,
}

impl View {
    pub fn new(key: String, view_type: ViewType) -> Self {
        Self {
            key,
            title: None,
            description: None,
            view_type,
            elements: Vec::new(),
            relationships: Vec::new(),
            auto_layout: None,
        }
    }

    pub fn add_element(&mut self, element_id: String) {
        if !self.elements.contains(&element_id) {
            self.elements.push(element_id);
        }
    }

    pub fn add_relationship(&mut self, relationship_id: String) {
        if !self.relationships.contains(&relationship_id) {
            self.relationships.push(relationship_id);
        }
    }
}
