#![windows_subsystem = "windows"]

use theframework::*;

pub mod editor;
pub mod ui;
pub mod pattern;
pub mod property;
pub mod hitrecord;
pub mod world;
pub mod camera;
pub mod buffer;
pub mod tile;
pub mod palette;
pub mod tool;
// pub mod script;
pub mod misc;
pub mod pbr_path;
pub mod material;
pub mod material_preview;
pub mod project;
pub mod sdf;
pub mod sdf3d;
pub mod shape;
pub mod value;
pub mod object;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "embedded/"]
#[exclude = ".txt"]
#[exclude = ".DS_Store"]
pub struct Embedded;

pub mod prelude {

    pub use theframework::TheContext;

    pub use crate::Embedded;
    pub use rustc_hash::FxHashMap;
    pub use uuid::Uuid;
    pub use serde::{Deserialize, Serialize};

    pub use maths_rs::prelude::*;

    pub use crate::ui::prelude::*;
    pub use crate::ui::UI;

    pub use crate::editor::{Editor, WORLD};
    pub use crate::pattern::*;
    pub use crate::property::*;
    pub use crate::hitrecord::HitRecord;
    pub use crate::world::World;
    pub use crate::camera::*;
    pub use crate::buffer::ColorBuffer;
    pub use crate::tile::Tile;
    pub use crate::palette::Palette;
    pub use crate::tool::Tool;
    pub use crate::object::Object;
    // pub use crate::script::*;
    pub use crate::misc::*;
    pub use crate::pbr_path::*;
    pub use crate::material::*;
    pub use crate::material_preview::MaterialPreview;
    pub use crate::project::*;
    pub use crate::sdf::*;
    pub use crate::sdf3d::{*, SDF3DType};
    pub use crate::shape::*;
    pub use crate::value::{*, Value::*};
}

use prelude::*;

fn main() {

    let editor = Editor::new();
    let mut app = TheApp::new();

    _ = app.run(Box::new(editor));
}
