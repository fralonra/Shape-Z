use crate::editor::NODEEDITOR;
use crate::prelude::*;
use theframework::prelude::*;

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CharacterUndoAtom {
    MapEdit(Box<Map>, Box<Map>),
}

impl CharacterUndoAtom {
    pub fn undo(&self, project: &mut Project, ui: &mut TheUI, ctx: &mut TheContext) {
        match self {
            CharacterUndoAtom::MapEdit(prev, _) => {
                for character in project.characters.values_mut() {
                    if character.map.id == prev.id {
                        character.map = *prev.clone();
                        character.map.clear_temp();
                        NODEEDITOR
                            .write()
                            .unwrap()
                            .force_update(ctx, &mut character.map);
                        break;
                    }
                }
                NODEEDITOR
                    .write()
                    .unwrap()
                    .set_selected_node_ui(project, ui, ctx, false);
            }
        }
    }
    pub fn redo(&self, project: &mut Project, ui: &mut TheUI, ctx: &mut TheContext) {
        match self {
            CharacterUndoAtom::MapEdit(_, next) => {
                for character in project.characters.values_mut() {
                    if character.map.id == next.id {
                        character.map = *next.clone();
                        character.map.clear_temp();
                        NODEEDITOR
                            .write()
                            .unwrap()
                            .force_update(ctx, &mut character.map);
                        break;
                    }
                }

                NODEEDITOR
                    .write()
                    .unwrap()
                    .set_selected_node_ui(project, ui, ctx, false);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterUndo {
    pub stack: Vec<CharacterUndoAtom>,
    pub index: isize,
}

impl Default for CharacterUndo {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterUndo {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            index: -1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.stack = vec![];
        self.index = -1;
    }

    pub fn has_undo(&self) -> bool {
        self.index >= 0
    }

    pub fn has_redo(&self) -> bool {
        if self.index >= -1 && self.index < self.stack.len() as isize - 1 {
            return true;
        }
        false
    }

    pub fn add(&mut self, atom: CharacterUndoAtom) {
        let to_remove = self.stack.len() as isize - self.index - 1;
        for _i in 0..to_remove {
            self.stack.pop();
        }
        self.stack.push(atom);
        self.index += 1;
    }

    pub fn undo(&mut self, project: &mut Project, ui: &mut TheUI, ctx: &mut TheContext) {
        if self.index >= 0 {
            self.stack[self.index as usize].undo(project, ui, ctx);
            self.index -= 1;
        }
    }

    pub fn redo(&mut self, project: &mut Project, ui: &mut TheUI, ctx: &mut TheContext) {
        if self.index < self.stack.len() as isize - 1 {
            self.index += 1;
            self.stack[self.index as usize].redo(project, ui, ctx);
        }
    }

    pub fn truncate_to_limit(&mut self, limit: usize) {
        if self.stack.len() > limit {
            let excess = self.stack.len() - limit;

            // Remove the oldest `excess` entries from the front
            self.stack.drain(0..excess);

            // Adjust the index accordingly
            self.index -= excess as isize;

            // Clamp to -1 minimum in case we truncated everything
            if self.index < -1 {
                self.index = -1;
            }
        }
    }
}
