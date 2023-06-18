use theframework::*;
use crate::prelude::*;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref WORLD : Mutex<World> = Mutex::new(World::new());
    // pub static ref WORLD : Mutex<World> = Mutex::new(World::new());
}

#[derive(PartialEq, Debug, Clone)]
enum EditorMode {
    CameraPan,
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
        Self {

            ui          : UI::new(),
            context     : Context::new(),

            buffer      : ColorBuffer::new(10, 10),

            click_drag  : None,

            ui_drag     : false,

            path_iter   : 0,
            path_max    : 100,
        }
    }

    /// Draw a circle in the middle of the window
    fn draw(&mut self, pixels: &mut [u8], ctx: &mut TheContext) {

        self.context.width = ctx.width;
        self.context.height = ctx.height;

        //println!("update {}", self.path_iter);

        // Make sure world has the correct size
        let world_width = ctx.width - self.ui.settings_width -  self.ui.palettebar_width;
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
    fn touch_down(&mut self, x: f32, y: f32, ctx: &mut TheContext) -> bool {

        self.ui_drag = false;
        if self.ui.touch_down(x, y, &mut self.context) {
            self.process_cmds();
            self.ui_drag = true;
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

            let hit = WORLD.lock().unwrap().hit_at(self.to_world(vec2f(x, y)), &self.buffer);

            if let Some(mut hit) = hit {
                hit.compute_side();
                WORLD.lock().unwrap().curr_tool = self.context.curr_tool.clone();
                self.context.curr_tool.hit(&self.context.engine, hit);
                consumed = true;
            }
        }
        return true;
    }


    /// Click / touch at the given position, check if we clicked inside the circle
    fn touch_dragged(&mut self, x: f32, y: f32, ctx: &mut TheContext) -> bool {

        if self.ui_drag && self.ui.touch_dragged(x, y, &mut self.context) {
            self.process_cmds();
            self.ui_drag = true;
            true
        } else {

            if let Some(mut click_drag) = self.click_drag {

                let xx = (click_drag.0 - x) / 100.0;
                let yy = (y - click_drag.1) / 100.0;

                click_drag.0 = x;
                click_drag.1 = y;
                self.click_drag = Some(click_drag);

                //self.world.camera.set_top_down_angle(10.0, 10.0, vec3f(0.0, 0.0, 5.0));

                WORLD.lock().unwrap().camera.move_by(xx, yy);
                WORLD.lock().unwrap().needs_update = true;
                return true;
            }

            false
        }
    }

    fn touch_up(&mut self, x: f32, y: f32, ctx: &mut TheContext) -> bool {
        self.ui_drag = false;

        if self.ui.touch_up(x, y, &mut self.context) {
            self.process_cmds();
            true
        } else {
            false
        }
    }

    fn needs_update(&mut self, ctx: &mut TheContext) -> bool {
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
                    for (w_index, v) in self.context.curr_tool.widget_values.clone().iter().enumerate() {
                        match v {
                            WidgetValue::Color(name, i) => {
                                self.context.curr_tool.widget_values[w_index] = WidgetValue::Color(name.clone(), *index);
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    /// Convert a screen space coordinate to a world coordinate
    fn to_world(&self, mut pos: Vec2f) -> Vec2f {
        pos.x -= self.ui.palettebar_width as f32;
        pos.y -= self.ui.toolbar_height as f32;
        pos
    }
}