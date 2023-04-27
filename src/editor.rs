use theframework::*;
use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
enum EditorMode {
    CameraPan,
}

pub struct Editor {

    ui                  : UI,
    context             : Context,

    world               : World,
    buffer              : ColorBuffer,

    click_drag          : Option<(f32, f32)>,

    ui_drag             : bool,
}

impl TheTrait for Editor {
    fn new() -> Self where Self: Sized {
        Self {

            ui          : UI::new(),
            context     : Context::new(),

            world       : World::new(),
            buffer      : ColorBuffer::new(10, 10),

            click_drag  : None,

            ui_drag     : false,
        }
    }

    /// Draw a circle in the middle of the window
    fn draw(&mut self, pixels: &mut [u8], ctx: &TheContext) {

        self.context.width = ctx.width;
        self.context.height = ctx.height;

        // Make sure world has the correct size
        let world_width = ctx.width - self.ui.settings_width -  self.ui.modebar_width;
        let world_height = ctx.height - self.ui.browser_height - self.ui.toolbar_height;

        if self.buffer.width != world_width|| self.buffer.height != world_height {
            self.buffer = ColorBuffer::new(world_width, world_height);
            self.world.needs_update = true;
        }

        // Render world
        if self.world.needs_update {
            self.world.render(&mut self.buffer, &self.context);
            self.world.needs_update = false;
        }
        self.buffer.convert_to_u8_at(pixels, (self.ui.modebar_width, self.ui.toolbar_height, ctx.width, ctx.height));

        // Draw UI
        self.ui.draw(pixels, &mut self.context, &self.world, ctx);
    }

    /// Click / touch at the given position, check if we clicked inside the circle
    fn touch_down(&mut self, x: f32, y: f32) -> bool {

        self.ui_drag = false;
        if self.ui.touch_down(x, y, &mut self.context) {
            self.process_cmds();
            self.ui_drag = true;
            return true;
        } else {

            self.click_drag = Some((x, y));

            if let Some(key) = self.world.key_at(vec2f(x, y), &self.buffer) {
                if Some(key) != self.context.curr_key {
                    if let Some(tile) = self.world.get_tile(key) {
                        self.context.curr_tile = tile;
                        self.context.curr_key = Some(key);
                        self.ui.update(&mut self.context);
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

            if let Some(mut click_drag) = self.click_drag {

                let xx = (click_drag.0 - x) / 100.0;
                let yy = (y - click_drag.1) / 100.0;

                click_drag.0 = x;
                click_drag.1 = y;
                self.click_drag = Some(click_drag);

                //self.world.camera.set_top_down_angle(10.0, 10.0, vec3f(0.0, 0.0, 5.0));

                self.world.camera.move_by(xx, yy);
                self.world.needs_update = true;
                return true;
            }

            false
        }
    }

    fn touch_up(&mut self, x: f32, y: f32) -> bool {
        self.ui_drag = false;

        if self.ui.touch_up(x, y, &mut self.context) {
            self.process_cmds();
            true
        } else {
            false
        }
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