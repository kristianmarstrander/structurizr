//! Relationship definitions

/// Relationship between two elements
#[derive(Debug, Clone)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub destination_id: String,
    pub description: String,
    pub technology: Option<String>,
    pub tags: Vec<String>,
}

impl Relationship {
    pub fn new(
        id: String,
        source_id: String,
        destination_id: String,
        description: String,
    ) -> Self {
        Self {
            id,
            source_id,
            destination_id,
            description,
            technology: None,
            tags: vec!["Relationship".to_string()],
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}
