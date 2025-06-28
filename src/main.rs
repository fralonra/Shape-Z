#![windows_subsystem = "windows"]

use theframework::*;

pub mod context;
pub mod editor;
pub mod misc;
pub mod modeleditor;
pub mod node;
pub mod nodeeditor;
pub mod project;
pub mod toollist;
pub mod tools;
pub mod utils;
pub mod voxel;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "embedded/"]
#[exclude = "*.DS_Store"]
pub struct Embedded;

pub type F = f32;
pub const F_PI: F = std::f32::consts::PI;
pub const F_TAU: F = std::f32::consts::TAU;
pub const F_FRAC_PI_2: F = std::f32::consts::FRAC_PI_2;
pub const F_FRAC_1_PI: F = std::f32::consts::FRAC_1_PI;
pub const F_E: F = std::f32::consts::E;
pub const F_SQRT_2: F = std::f32::consts::SQRT_2;
pub const F_MIN: F = f32::MIN;
pub const F_MAX: F = f32::MAX;

pub type Color = [F; 4];

#[allow(ambiguous_glob_reexports)]
pub mod prelude {
    pub use serde::{Deserialize, Serialize};

    pub use std::sync::{LazyLock, RwLock};
    pub use theframework::prelude::*;

    pub use crate::context::*;
    pub use crate::misc::UpdateTracker;
    pub use crate::modeleditor::*;
    pub use crate::nodeeditor::*;
    pub use crate::project::*;
    pub use crate::toollist::*;
    pub use crate::{Color, F};

    pub use crate::voxel::camera::Camera;
    pub use crate::voxel::camera::iso::Iso;
    pub use crate::voxel::camera::orbit::Orbit;
    pub use crate::voxel::camera::pinhole::Pinhole;
    pub use crate::voxel::grid::VoxelGrid;
    pub use crate::voxel::palette::{Material, Palette};
    pub use crate::voxel::ray::Ray;
    pub use crate::voxel::renderbuffer::RenderBuffer;
    pub use crate::voxel::renderer::Renderer;
    // pub use crate::voxel::renderer::editshader::EditShader;
    pub use crate::voxel::renderer::pbr::PBR;
    pub use crate::voxel::tile::Tile;
    pub use crate::voxel::{Coord, Face, HitRecord, HitType};

    pub use crate::node::graph::*;
    pub use crate::node::nodefx::*;

    pub use crate::tools::{Tool, ToolEvent};

    /*
    pub use crate::codeeditor::*;
    pub use crate::effectpicker::*;
    pub use crate::mapeditor::*;
    pub use crate::materialpicker::*;
    pub use crate::misc::*;
    pub use crate::panels::*;
    // pub use crate::previewview::*;
    pub use crate::shapepicker::*;
    pub use crate::sidebar::*;
    pub use crate::tilemapeditor::*;
    pub use crate::tilepicker::*;
    pub use crate::toollist::*;
    pub use crate::undo::material_undo::*;
    pub use crate::undo::palette_undo::*;
    pub use crate::undo::region_undo::*;
    pub use crate::undo::*;
    pub use crate::utils::*;

    pub use crate::tools::code::CodeTool;
    pub use crate::tools::game::GameTool;
    pub use crate::tools::linedef::LinedefTool;
    pub use crate::tools::sector::SectorTool;
    pub use crate::tools::selection::SelectionTool;
    pub use crate::tools::tileset::TilesetTool;
    pub use crate::tools::vertex::VertexTool;

    pub use crate::tools::*;

    pub use crate::configeditor::ConfigEditor;
    pub use crate::customcamera::{CustomCamera, CustomMoveAction};
    pub use crate::infoviewer::InfoViewer;
    pub use crate::nodeeditor::{NodeContext, NodeEditor};
    pub use crate::rendereditor::{RenderEditor, RenderMoveAction};
    pub use crate::worldeditor::WorldEditor;
    pub use toml::Table;
    */
}

use crate::editor::Editor;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    // unsafe {
    //     std::env::set_var("RUST_BACKTRACE", "1");
    // }

    let editor = Editor::new();
    let mut app = TheApp::new();
    app.set_cmd_line_args(args);

    let () = app.run(Box::new(editor));
}
