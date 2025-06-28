use crate::{
    editor::{NODEEDITOR, PALETTE},
    prelude::*,
};

pub struct EditTool {
    id: TheId,
}

impl Tool for EditTool {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            id: TheId::named("Edit Tool"),
        }
    }
    fn id(&self) -> TheId {
        self.id.clone()
    }
    fn info(&self) -> String {
        str!("Edit Tool.")
    }
    fn icon_name(&self) -> String {
        str!("move")
    }
    fn accel(&self) -> Option<char> {
        Some('e')
    }

    fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        let mut redraw = false;
        #[allow(clippy::single_match)]
        match event {
            TheEvent::PaletteIndexChanged(id, index) => {
                if ToolMode::Palette == context.mode {
                    let palette = PALETTE.read().unwrap();
                    let mut editor = NODEEDITOR.write().unwrap();
                    editor.set_graph(
                        NodeContext::Color(*index as u8),
                        palette.graphs[*index as usize].clone(),
                        ui,
                        ctx,
                    );
                    redraw = true;
                }
            }
            _ => {}
        }
        redraw
    }
}
