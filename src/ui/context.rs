use crate::{prelude::*, tool::ToolRole};
use fontdue::Font;

#[derive(PartialEq, Clone, Debug)]
pub enum Command {
    None,
    ColorIndexChanged(u8),
    MaterialIndexChanged(u8),
    EditStateSwitched,
}

pub struct Context {
    //pub shapes              : Vec<Tile>,
    //pub patterns            : Vec<Box<dyn Pattern>>,

    pub width                   : usize,
    pub height                  : usize,

    pub color_button            : [u8;4],
    pub color_widget            : [u8;4],
    pub color_toolbar           : [u8;4],
    pub color_selected          : [u8;4],
    pub color_text              : [u8;4],
    pub color_orange            : [u8;4],
    pub color_green             : [u8;4],
    pub color_red               : [u8;4],
    pub color_blue              : [u8;4],
    pub color_white             : [u8;4],
    pub color_black             : [u8;4],

    pub curr_tile               : Tile,
    pub curr_key                : Option<Vec3i>,

    pub curr_tool               : Tool,
    pub curr_tool_role          : ToolRole,
    pub curr_tools              : Vec<String>,

    pub cmd                     : Option<Command>,

    pub palette                 : Palette,
    pub materials               : Vec<Material>,

    pub font                    : Option<Font>,
    pub icons                   : FxHashMap<String, (Vec<u8>, u32, u32)>,

    pub edit_state              : bool,

    // Tools

    pub engine                  : rhai::Engine,
    pub tools                   : FxHashMap<String, Tool>,

    pub curr_color_index        : u8,
    pub curr_material_index     : u8

}

impl Context {

    pub fn new() -> Self {

        let mut palette = Palette::new();

        let mut curr_tool = Tool::new("".into());
        let mut curr_tools = vec![];

        let mut tools : FxHashMap<String, Tool> = FxHashMap::default();
        let mut engine = setup_engine();

        // Load Font

        let mut font : Option<Font> = None;
        let mut icons : FxHashMap<String, (Vec<u8>, u32, u32)> = FxHashMap::default();

        for file in Embedded::iter() {
            let name = file.as_ref();
            if name.starts_with("fonts/") {
                if let Some(font_bytes) = Embedded::get(name) {
                    if let Some(f) = Font::from_bytes(font_bytes.data, fontdue::FontSettings::default()).ok() {
                        font = Some(f);
                    }
                }
            } else
            if name.starts_with("icons/") {
                if let Some(file) = Embedded::get(name) {
                    let data = std::io::Cursor::new(file.data);

                    let decoder = png::Decoder::new(data);
                    if let Ok(mut reader) = decoder.read_info() {
                        let mut buf = vec![0; reader.output_buffer_size()];
                        let info = reader.next_frame(&mut buf).unwrap();
                        let bytes = &buf[..info.buffer_size()];

                        let mut cut_name = name.replace("icons/", "");
                        cut_name = cut_name.replace(".png", "");
                        icons.insert(cut_name.to_string(), (bytes.to_vec(), info.width, info.height));
                    }
                }
            } else
            if name.starts_with("tools/") {
                if let Some(bytes) = Embedded::get(name) {
                    if let Some(string) = std::str::from_utf8(bytes.data.as_ref()).ok() {
                        //println!("{}", string);

                        let mut tool = Tool::new(string.into());

                        tool.init(&mut engine, name);
                        let name = tool.name();
                        //println!("{}", name);

                        if name == "Add" {
                            curr_tool = tool.clone();
                        }

                        curr_tools.push(name.clone());
                        tools.insert(name, tool);
                    }
                }
            } else {
                if name == "aurora.txt" {
                    if let Some(bytes) = Embedded::get(name) {
                        if let Some(string) = std::str::from_utf8(bytes.data.as_ref()).ok() {
                            palette.load_from_txt(string.to_string())
                        }
                    }
                }
            }
        }

        let mut materials : Vec<Material> = vec![];

        for metallic in 0..15 {
            for roughness in 0..15 {

                let mut material = Material::new(vec3f(0.5, 0.5, 0.5));
                material.metallic = if metallic == 14 { 1.0 } else { (1.0 / 15.0) * metallic as f32 };
                material.roughness = if roughness == 14 { 1.0 } else { (1.0 / 15.0) * roughness as f32 };

                if metallic > 5 {
                    material.emission = vec3f(2.0, 2.0, 2.0);
                }

                //println!("{} {}", material.roughness, material.metallic);

                materials.push(material);
            }
        }

        //println!("{}", materials.len());

        Self {
            // shapes,
            // patterns,

            width               : 0,
            height              : 0,

            color_button        : [53, 53, 53, 255],
            color_selected      : [135, 135, 135, 255],
            color_widget        : [24, 24, 24, 255],
            color_toolbar       : [29, 29, 29, 255],
            color_text          : [244, 244, 244, 255],
            color_orange        : [188, 68, 34, 255],
            color_green         : [10, 93, 80, 255],
            color_red           : [207, 55, 54, 255],
            color_blue          : [27, 79, 136, 255],
            color_white         : [255, 255, 255, 255],
            color_black         : [0, 0, 0, 255],

            curr_tile           : Tile::new(9),
            curr_tool_role      : ToolRole::Voxel,

            curr_key            : None,

            curr_tool,
            curr_tools,

            cmd                 : None,

            palette,
            materials,

            font,
            icons,

            edit_state          : true,

            engine,
            tools,

            curr_color_index    : 0,
            curr_material_index : 0,
        }
    }
}