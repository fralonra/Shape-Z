use theframework::*;
use crate::prelude::*;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref WORLD : Mutex<World> = Mutex::new(World::new());
    pub static ref TOOL : Mutex<Box<dyn Widget>> = Mutex::new(Box::new(ExtrusionTool::new()));
}

#[derive(PartialEq, Debug, Clone)]
enum EditorMode {
}

pub struct Editor {

    ui                  : UI,
    context             : Context,

    buffer              : ColorBuffer,

    click_drag          : Option<(f32, f32)>,

    ui_drag             : bool,

    path_iter           : i32,
    path_max            : i32,
}

impl TheTrait for Editor {
    fn new() -> Self where Self: Sized {

        WORLD.lock().unwrap().camera.center.x = 0.5;
        WORLD.lock().unwrap().camera.center.y = 0.5;
        WORLD.lock().unwrap().camera.compute_orbit(Vec2f::zero());

        Self {

            ui          : UI::new(),
            context     : Context::new(),

            buffer      : ColorBuffer::new(10, 10),

            click_drag  : None,

            ui_drag     : false,

            path_iter   : 0,
            path_max    : 300,
        }
    }

    /// Draw a circle in the middle of the window
    fn draw(&mut self, pixels: &mut [u8], ctx: &mut TheContext) {

        self.context.width = ctx.width;
        self.context.height = ctx.height;

        //println!("update {}", self.path_iter);

        // Make sure world has the correct size
        let world_width = ctx.width - self.ui.settings_width -  self.ui.palettebar_width - self.ui.shape_selector_width;
        let world_height = ctx.height - self.ui.browser_height - self.ui.toolbar_height;

        if self.buffer.width != world_width|| self.buffer.height != world_height {
            self.buffer = ColorBuffer::new(world_width, world_height);
            WORLD.lock().unwrap().needs_update = true;
        }

        // Render world
        if WORLD.lock().unwrap().needs_update || self.path_iter < self.path_max {
            WORLD.lock().unwrap().render(&mut self.buffer, &self.context, self.path_iter);

            if WORLD.lock().unwrap().needs_update {
                self.path_iter = 0;
            } else {
                self.path_iter += 1;
            }

            WORLD.lock().unwrap().needs_update = false;
        }

        self.buffer.convert_to_u8_at(pixels, (self.ui.palettebar_width, self.ui.toolbar_height, ctx.width, ctx.height));

        // Draw UI
        self.ui.draw(pixels, &mut self.context, &WORLD.lock().unwrap(), ctx);
    }

    /// Click / touch at the given position, check if we clicked inside the circle
    fn touch_down(&mut self, x: f32, y: f32, _ctx: &mut TheContext) -> bool {

        self.ui_drag = false;

        if self.ui.contains(x, y) {
            self.ui_drag = true;
            if self.ui.touch_down(x, y, &mut self.context, &WORLD.lock().unwrap()) {
                self.process_cmds();
                return true;
            }
        } else {

            self.click_drag = Some((x, y));

            let mut consumed = false;

            /*
            match self.context.curr_mode {
                Mode::Select => {

                    if let Some(world) = WORLD.lock().ok() {
                        if let Some(hit) = world.hit_at(self.to_world(vec2f(x, y)), &self.buffer) {
                            // println!("{:?}", hit);
                            let key = hit.key;
                            if Some(key) != self.context.curr_key {
                                if let Some(tile) = world.get_tile(key) {
                                    self.context.curr_tile = tile;
                                    self.context.curr_key = Some(key);
                                    self.ui.update(&mut self.context);
                                    consumed = true;
                                }
                            }
                        } else {
                            // println!("None");
                            if self.context.curr_key.is_some() {
                                self.context.curr_key = None;
                                self.ui.update(&mut self.context);
                                consumed = true;
                            }
                        }
                    }

                },
                Mode::Edit => {

                    let hit = WORLD.lock().unwrap().hit_at(self.to_world(vec2f(x, y)), &self.buffer);

                    if let Some(mut hit) = hit {
                        hit.compute_side();
                        self.context.curr_tool.hit(&self.context.engine, hit);
                        consumed = true;
                    }

                    // if let Some(key) = self.context.curr_key {
                    //     self.context.curr_tool.apply(&self.context.engine, key);
                    //     return true;
                    // }
                },
                _ => {}
            }*/

            let hit = WORLD.lock().unwrap().hit_at(self.to_world(vec2f(x, y)), &self.buffer, self.context.iso_state);

            if let Some(mut hit) = hit {
                //self.context.curr_keys = vec!(hit.key);
                hit.compute_side();
                //WORLD.lock().unwrap().curr_tool = self.context.curr_tool.clone();
                //self.context.curr_tool.hit(&self.context.engine, hit);
                //WORLD.lock().unwrap().apply(hit.key, hit.tile_key, &self.context.curr_keys);
                //WORLD.lock().unwrap().apply_tool_hit(hit, &self.context.curr_keys);
                let mut tool = TOOL.lock().unwrap();
                tool.hit(hit, &self.context.curr_keys);
                consumed = true;
            } else {
                /*
                if self.context.curr_keys.is_empty() == false {
                    self.context.curr_keys = vec![];
                    WORLD.lock().unwrap().needs_update = true;
                    consumed = true;
                }*/
            }

            return consumed;
        }
        false
    }

    /// Click / touch at the given position, check if we clicked inside the circle
    fn touch_dragged(&mut self, x: f32, y: f32, _ctx: &mut TheContext) -> bool {

        if self.ui_drag {
            if self.ui.touch_dragged(x, y, &mut self.context) {
                self.process_cmds();
                self.ui_drag = true;
                return true;
            }
        } else {

            if let Some(mut click_drag) = self.click_drag {

                let xx = (x - click_drag.0) / 200.0;
                let yy = (y - click_drag.1) / 400.0;

                //let xx = x - self.ui.palettebar_width as f32;
                //let yy = y - self.ui.toolbar_height as f32;

                click_drag.0 = x;
                click_drag.1 = y;
                self.click_drag = Some(click_drag);

                WORLD.lock().unwrap().camera.compute_orbit(vec2f(xx, yy));

                //WORLD.lock().unwrap().camera.set_top_down_angle(10.0, 2.0, vec3f(0.0, 0.0, 0.0));

                //WORLD.lock().unwrap().camera.rotate(xx, 0.0);
                WORLD.lock().unwrap().needs_update = true;
                return true;
            }
        }
        false
    }

    fn touch_up(&mut self, x: f32, y: f32, _ctx: &mut TheContext) -> bool {
        self.ui_drag = false;

        if self.ui.touch_up(x, y, &mut self.context) {
            self.process_cmds();
            true
        } else {
            false
        }
    }

    fn key_down(&mut self, char: Option<char>, key: Option<WidgetKey>, ctx: &mut TheContext) -> bool {
        self.ui.key_down(char, key, &mut self.context)
    }

    fn needs_update(&mut self, _ctx: &mut TheContext) -> bool {
        if self.path_iter < self.path_max {
            true
        } else {
            false
        }
    }

}

pub trait MyEditor {
    fn process_cmds(&mut self);
    fn to_world(&self, pos: Vec2f) -> Vec2f;
}

impl MyEditor for Editor {
    /// Process possible UI commands
    fn process_cmds(&mut self) {
        if let Some(cmd) = &self.context.cmd {
            match cmd {
                Command::ColorIndexChanged(index) => {
                    self.context.curr_color_index = *index;
                    /*
                    for (w_index, v) in self.context.curr_tool.widget_values.clone().iter().enumerate() {
                        match v {
                            WidgetValue::Color(name, _i) => {
                                self.context.curr_tool.widget_values[w_index] = WidgetValue::Color(name.clone(), *index);
                            },
                            _ => {},
                        }
                    }*/
                },
                Command::MaterialIndexChanged(_index) => {
                    /*
                    self.context.curr_material_index = *index;
                    for (w_index, v) in self.context.curr_tool.widget_values.clone().iter().enumerate() {
                        match v {
                            WidgetValue::Material(name, _i) => {
                                self.context.curr_tool.widget_values[w_index] = WidgetValue::Material(name.clone(), *index);
                            },
                            _ => {},
                        }
                    }*/
                },
                Command::EditStateSwitched => {
                    self.context.edit_state = self.ui.get_edit_state();
                    WORLD.lock().unwrap().needs_update = true;
                },
                Command::IsoStateSwitched => {
                    self.context.iso_state = !self.ui.get_iso_state();
                    WORLD.lock().unwrap().needs_update = true;
                },
                Command::TileSelected(x, y, z) => {
                    //if WORLD.lock().unwrap().project.tiles.contains_key(&(*x, *y, *z)) {
                        if self.context.curr_keys.contains(&vec3i(*x, *y, *z)) {
                            if let Some(index) = self.context.curr_keys.iter().position(|&k| k == vec3i(*x, *y, *z)) {
                                self.context.curr_keys.remove(index);
                            }
                        } else {
                            self.context.curr_keys.push(vec3i(*x, *y, *z));
                        }
                    //}
                },
                Command::TileFocusSelected(x, y, z) => {
                    if WORLD.lock().unwrap().project.tiles.contains_key(&(*x, *y, *z)) {
                        if self.context.curr_keys.contains(&vec3i(*x, *y, *z)) {
                            if let Some(index) = self.context.curr_keys.iter().position(|&k| k == vec3i(*x, *y, *z)) {
                                self.context.curr_keys.remove(index);
                            }
                        } else {
                            self.context.curr_keys.push(vec3i(*x, *y, *z));
                        }
                        WORLD.lock().unwrap().set_focus(vec3i(*x, *y, *z));
                    }
                },
                Command::CreateTile(x, y, z) => {
                    let mut world = WORLD.lock().unwrap();

                    if world.project.tiles.contains_key(&(*x, *y, *z)) == false {
                        let mut tile = Tile::new(Project::tile_size());
                        tile.build_aabb();
                        world.project.tiles.insert((*x, *y, *z), tile);
                        world.project.build_aabb();
                        world.needs_update = true;
                    }
                },
                Command::DeleteTile(x, y, z) => {
                    let mut world = WORLD.lock().unwrap();

                    if world.project.tiles.contains_key(&(*x, *y, *z)) == true {
                        world.project.tiles.remove(&(*x, *y, *z));
                        world.project.build_aabb();
                        world.needs_update = true;
                    }
                },
                Command::ApplyTool => {
                    // let script = self.context.code_editor.get_text();
                    // println!("{}", script);
                    // WORLD.lock().unwrap().curr_tool = self.context.curr_tool.clone();
                    // self.context.curr_tool.apply(&self.context.engine, self.context.curr_keys.clone());
                    WORLD.lock().unwrap().apply(&self.context.curr_keys);
                },
                Command::SDFSelected(sdf_type) => {
                    TOOL.lock().unwrap().sdf_triggered(*sdf_type);
                    self.ui.update_settings(&mut self.context);
                    //WORLD.lock().unwrap().curr_tool = self.context.curr_tool.clone();
                    //self.context.curr_tool.apply(&self.context.engine, self.context.curr_keys.clone());
                    //WORLD.lock().unwrap().apply(hit.key, hit.tile_key, &self.context.curr_keys);
                }
                _ => {}
            }
        }
        self.context.cmd = None;
    }

    /// Convert a screen space coordinate to a world coordinate
    fn to_world(&self, mut pos: Vec2f) -> Vec2f {
        pos.x -= self.ui.palettebar_width as f32;
        pos.y -= self.ui.toolbar_height as f32;
        pos
    }
}