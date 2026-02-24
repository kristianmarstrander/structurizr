//! Model module containing data structures for Structurizr workspace

mod element;
mod relationship;
mod style;
mod view;

pub use element::{Element, ElementType};
pub use relationship::Relationship;
pub use style::{Border, ElementStyle, RelationshipStyle, Shape, Styles};
pub use view::{AutoLayout, Direction, View, ViewType};

use std::collections::HashMap;

/// Workspace represents the top-level Structurizr workspace
#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String,
    pub description: String,
    pub elements: HashMap<String, Element>,
    pub relationships: Vec<Relationship>,
    pub views: Vec<View>,
    pub styles: Styles,
}

impl Workspace {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            elements: HashMap::new(),
            relationships: Vec::new(),
            views: Vec::new(),
            styles: Styles::default(),
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(element.id.clone(), element);
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    pub fn add_view(&mut self, view: View) {
        self.views.push(view);
    }

    pub fn get_element(&self, id: &str) -> Option<&Element> {
        self.elements.get(id)
    }
}
