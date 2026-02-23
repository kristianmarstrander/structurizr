//! Element definitions

use std::collections::HashMap;

/// Element types in the Structurizr model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    Person,
    SoftwareSystem,
    Container,
    Component,
    DeploymentNode,
    InfrastructureNode,
}

/// An element in the model (Person, SoftwareSystem, Container, Component)
#[derive(Debug, Clone)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub element_type: ElementType,
    pub tags: Vec<String>,
    pub technology: Option<String>,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub properties: HashMap<String, String>,
}

impl Element {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        element_type: ElementType,
    ) -> Self {
        let mut tags = vec!["Element".to_string()];
        
        // Add default tags based on type
        match element_type {
            ElementType::Person => tags.push("Person".to_string()),
            ElementType::SoftwareSystem => tags.push("Software System".to_string()),
            ElementType::Container => tags.push("Container".to_string()),
            ElementType::Component => tags.push("Component".to_string()),
            ElementType::DeploymentNode => tags.push("Deployment Node".to_string()),
            ElementType::InfrastructureNode => tags.push("Infrastructure Node".to_string()),
        }

        Self {
            id,
            name,
            description,
            element_type,
            tags,
            technology: None,
            parent_id: None,
            children: Vec::new(),
            properties: HashMap::new(),
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn add_child(&mut self, child_id: String) {
        self.children.push(child_id);
    }
}
