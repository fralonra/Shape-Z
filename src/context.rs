pub use rusterix::map::*;
use theframework::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ToolMode {
    Palette,
    Point,
    History,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
    pub mode: ToolMode,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            mode: ToolMode::Palette,
        }
    }
}
