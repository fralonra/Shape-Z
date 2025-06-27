use crate::prelude::*;

use NodeFXParam::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeContext {
    Empty,
    Color(u8),
}

use NodeContext::*;

pub struct NodeEditor {
    pub context: NodeContext,
    pub graph: NodeFXGraph,

    pub categories: FxHashMap<String, TheColor>,
}

#[allow(clippy::new_without_default)]
impl NodeEditor {
    pub fn new() -> Self {
        let mut categories: FxHashMap<String, TheColor> = FxHashMap::default();
        categories.insert("ShapeFX".into(), TheColor::from("#c49a00")); // Warm gold — represents pixel-level artistic material control
        categories.insert("Render".into(), TheColor::from("#e53935")); // Bright red — strong signal for core rendering pipeline
        categories.insert("Modifier".into(), TheColor::from("#00bfa5")); // Teal green — evokes transformation, procedural mesh edits
        categories.insert("FX".into(), TheColor::from("#7e57c2")); // Purple — expressive and magical for particles, screen fx, etc.
        categories.insert("Shape".into(), TheColor::from("#4285F4")); // Vivid blue — represents structure, geometric form, and stability

        Self {
            context: NodeContext::Empty,
            graph: NodeFXGraph::default(),
            categories,
        }
    }

    /// Set the context and graph.
    pub fn set_graph(&mut self, context: NodeContext, graph: NodeFXGraph, ui: &mut TheUI) {
        self.context = context;
        self.graph = graph;

        let canvas = self.to_canvas();
        ui.set_node_canvas("NodeView", canvas);
    }

    pub fn to_canvas(&mut self) -> TheNodeCanvas {
        let mut canvas = TheNodeCanvas {
            node_width: 136,
            selected_node: self.graph.selected_node,
            offset: self.graph.scroll_offset,
            connections: self.graph.connections.clone(),
            categories: self.categories.clone(),
            ..Default::default()
        };

        // let width = 111;
        // let height = 104;

        // let mut buffer = TheRGBABuffer::new(TheDim::sized(width as i32, height));

        for (index, node) in self.graph.nodes.iter().enumerate() {
            let n = TheNode {
                name: node.name(),
                position: node.position,
                inputs: node.inputs(),
                outputs: node.outputs(),
                preview: TheRGBABuffer::default(),
                supports_preview: true,
                preview_is_open: true,
                can_be_deleted: index != 0,
            };
            canvas.nodes.push(n);
        }

        canvas
    }
}
