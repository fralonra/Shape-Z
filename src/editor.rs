use theframework::*;
use crate::prelude::*;

pub struct Editor {

    ui                  : UI,
    context             : Context,

    world               : World,
    buffer              : ColorBuffer,

    ui_drag             : bool,
}

impl TheTrait for Editor {
    fn new() -> Self where Self: Sized {
        Self {

            ui          : UI::new(),
            context     : Context::new(),

            world       : World::new(),
            buffer      : ColorBuffer::new(10, 10),

            ui_drag     : false,
        }
    }

    /// Draw a circle in the middle of the window
    fn draw(&mut self, pixels: &mut [u8], ctx: &TheContext) {

        self.context.width = ctx.width;
        self.context.height = ctx.height;

        if self.buffer.width != ctx.width || self.buffer.height != ctx.height {
            self.buffer = ColorBuffer::new(ctx.width, ctx.height);
        }

        self.world.render(&mut self.buffer);
        self.buffer.convert_to_u8(pixels);

        self.ui.draw(pixels, &mut self.context, &self.world, ctx);
    }

    /// Click / touch at the given position, check if we clicked inside the circle
    fn touch_down(&mut self, button: i32, x: f32, y: f32) -> bool {

        self.ui_drag = false;
        if self.ui.touch_down(button, x, y, &mut self.context) {
            self.process_cmds();
            self.ui_drag = true;
            return true;
        } else {
            if let Some(key) = self.world.key_at(vec2f(x, y), &self.buffer) {
                if Some(key) != self.context.curr_key {
                    if let Some(tile) = self.world.get_tile(key) {
                        self.context.curr_tile = tile;
                        self.context.curr_key = Some(key);
                        return true;
                    }
                }
            } else {
                if self.context.curr_key.is_some() {
                    self.context.curr_key = None;
                    return true;
                }
            }
        }

        false
    }


    /// Click / touch at the given position, check if we clicked inside the circle
    fn touch_dragged(&mut self, x: f32, y: f32) -> bool {

        if self.ui_drag && self.ui.touch_dragged(x, y, &mut self.context) {
            self.process_cmds();
            self.ui_drag = true;
            true
        } else {
            false
        }
    }

    fn touch_up(&mut self, _x: f32, _y: f32) -> bool {
        false
    }


}

pub trait AddEditor {
    fn process_cmds(&mut self);
}

impl AddEditor for Editor {
    /// Process possible UI commands
    fn process_cmds(&mut self) {
        if let Some(cmd) = &self.context.cmd {
            match cmd {
                _ => {}
            }
        }
    }
}