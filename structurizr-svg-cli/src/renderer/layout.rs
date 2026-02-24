//! Layout algorithm for auto-positioning elements

use crate::model::{Direction, View, Workspace};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Layout {
    positions: HashMap<String, Position>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    /// Calculate positions for all elements in a view
    pub fn calculate_layout(
        &mut self,
        workspace: &Workspace,
        view: &View,
        direction: &Direction,
    ) {
        // Build graph
        let mut graph = DiGraph::new();
        let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
        
        // Add nodes
        for element_id in &view.elements {
            let idx = graph.add_node(element_id.clone());
            node_map.insert(element_id.clone(), idx);
        }
        
        // Add edges
        for rel in &workspace.relationships {
            if view.elements.contains(&rel.source_id) && view.elements.contains(&rel.destination_id) {
                if let (Some(&src_idx), Some(&dst_idx)) = (
                    node_map.get(&rel.source_id),
                    node_map.get(&rel.destination_id),
                ) {
                    graph.add_edge(src_idx, dst_idx, ());
                }
            }
        }
        
        // Simple layered layout
        self.layered_layout(&graph, &node_map, direction);
    }

    fn layered_layout(
        &mut self,
        graph: &DiGraph<String, ()>,
        node_map: &HashMap<String, NodeIndex>,
        direction: &Direction,
    ) {
        // Assign layers using BFS from nodes with no incoming edges
        let mut layers: HashMap<NodeIndex, usize> = HashMap::new();
        let mut max_layer = 0;
        
        // Find root nodes (no incoming edges)
        let mut roots = Vec::new();
        for node_idx in graph.node_indices() {
            if graph.neighbors_directed(node_idx, petgraph::Direction::Incoming).count() == 0 {
                roots.push(node_idx);
            }
        }
        
        // If no roots, just use all nodes
        if roots.is_empty() {
            roots = graph.node_indices().collect();
        }
        
        // BFS to assign layers
        let mut queue: Vec<(NodeIndex, usize)> = roots.iter().map(|&n| (n, 0)).collect();
        let mut visited = std::collections::HashSet::new();
        
        while let Some((node_idx, layer)) = queue.pop() {
            if visited.contains(&node_idx) {
                continue;
            }
            visited.insert(node_idx);
            
            layers.insert(node_idx, layer);
            max_layer = max_layer.max(layer);
            
            for neighbor in graph.neighbors_directed(node_idx, petgraph::Direction::Outgoing) {
                if !visited.contains(&neighbor) {
                    queue.push((neighbor, layer + 1));
                }
            }
        }
        
        // Count nodes per layer
        let mut layer_counts: HashMap<usize, usize> = HashMap::new();
        for &layer in layers.values() {
            *layer_counts.entry(layer).or_insert(0) += 1;
        }
        
        // Assign positions within layers
        let mut layer_positions: HashMap<usize, usize> = HashMap::new();
        
        let node_width = 450;
        let node_height = 300;
        let horizontal_spacing = 100;
        let vertical_spacing = 150;
        
        for (element_id, &node_idx) in node_map {
            let layer = layers.get(&node_idx).copied().unwrap_or(0);
            let position_in_layer = layer_positions.entry(layer).or_insert(0);
            
            let (x, y) = match direction {
                Direction::TopBottom => {
                    let x = (*position_in_layer as i32) * (node_width + horizontal_spacing) as i32 + horizontal_spacing as i32;
                    let y = (layer as i32) * (node_height + vertical_spacing) as i32 + vertical_spacing as i32;
                    (x, y)
                }
                Direction::BottomTop => {
                    let x = (*position_in_layer as i32) * (node_width + horizontal_spacing) as i32 + horizontal_spacing as i32;
                    let y = ((max_layer - layer) as i32) * (node_height + vertical_spacing) as i32 + vertical_spacing as i32;
                    (x, y)
                }
                Direction::LeftRight => {
                    let x = (layer as i32) * (node_width + horizontal_spacing) as i32 + horizontal_spacing as i32;
                    let y = (*position_in_layer as i32) * (node_height + vertical_spacing) as i32 + vertical_spacing as i32;
                    (x, y)
                }
                Direction::RightLeft => {
                    let x = ((max_layer - layer) as i32) * (node_width + horizontal_spacing) as i32 + horizontal_spacing as i32;
                    let y = (*position_in_layer as i32) * (node_height + vertical_spacing) as i32 + vertical_spacing as i32;
                    (x, y)
                }
            };
            
            self.positions.insert(element_id.clone(), Position { x, y });
            *position_in_layer += 1;
        }
    }

    pub fn get_position(&self, element_id: &str) -> Option<&Position> {
        self.positions.get(element_id)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}
