use theframework::*;

pub mod editor;
pub mod ui;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "embedded/"]
#[exclude = ".txt"]
#[exclude = ".DS_Store"]
pub struct Embedded;

pub mod prelude {

    pub use crate::Embedded;
    pub use rustc_hash::FxHashMap;

    pub use maths_rs::prelude::*;

    pub use crate::editor::Editor;
    pub use crate::ui::prelude::*;
}

use prelude::*;

fn main() {

    let circle = Editor::new();
    let mut app = TheApp::new();

    _ = app.run(Box::new(circle));
}
