use crate::prelude::*;
use theframework::prelude::*;
use vek::Vec2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFXGraph {
    pub id: Uuid,
    pub nodes: Vec<NodeFX>,

    /// The node connections: Source node index, source terminal, dest node index, dest terminal
    pub connections: Vec<(u16, u8, u16, u8)>,

    pub selected_node: Option<usize>,

    pub scroll_offset: Vec2<i32>,
    pub zoom: f32,
}

impl Default for NodeFXGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeFXGraph {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            nodes: vec![],
            connections: vec![],
            selected_node: None,
            scroll_offset: Vec2::zero(),
            zoom: 1.0,
        }
    }
}
