
use crate::prelude::*;

pub struct SwitchButton {
    rect                : Rect,

    text_list           : Vec<String>,
    cmd                 : Option<Command>,

    has_state           : bool,
    state               : bool,
}

impl Widget for SwitchButton {

    fn new() -> Self {

        Self {
            rect        : Rect::empty(),
            text_list   : vec![],
            cmd         : None,
            has_state   : true,
            state       : false,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn set_text_list(&mut self, text_list: Vec<String>) {
        self.text_list = text_list;
    }

    fn set_cmd(&mut self, cmd: Command) {
        self.cmd = Some(cmd);
    }

    fn set_has_state(&mut self, state: bool) {
        self.state = state;
        self.has_state = true;
    }

    fn get_state(&self) -> bool {
        self.state
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, _world: &World, ctx: &TheContext) {
        let r = self.rect.to_usize();
        let rounding = (r.3 as f32 / 2.0 + 0.5).ceil();

        let mut color = if self.state { &context.color_white } else { &context.color_black };

        ctx.draw.rounded_rect(pixels, &(r.0 + 1, r.1, r.2 - 1, r.3), context.width, &color, &(rounding, rounding, rounding, rounding));

        color = if !self.state { &context.color_white } else { &context.color_black };

        let mut left_r = r.clone();
        left_r.2 = left_r.2 / 2 + 5;
        ctx.draw.rounded_rect(pixels, &left_r, context.width, &color, &(rounding, rounding, rounding, rounding));

        if let Some(font) = &context.font {

            color = if self.state { &context.color_white } else { &context.color_black };

            ctx.draw.blend_text_rect(pixels, &left_r, context.width, &font, r.3 as f32 / 2.0, &self.text_list[0], &color, theframework::thedraw2d::TheTextAlignment::Center);

            color = if !self.state { &context.color_white } else { &context.color_black };

            let mut right_r = r.clone();
            right_r.0 = right_r.0 + right_r.2 / 2;
            right_r.2 = right_r.2 / 2;

            ctx.draw.blend_text_rect(pixels, &right_r, context.width, &font, r.3 as f32 / 2.0, &self.text_list[1], &color, theframework::thedraw2d::TheTextAlignment::Center);
        }
    }

    fn contains(&mut self, x: f32, y: f32) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {
            true
        } else {
            false
        }
    }

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, _world: &World) -> bool {

        if self.rect.is_inside((x as usize, y as usize)) {
            context.cmd = self.cmd.clone();
            if x as usize > self.rect.x + self.rect.width / 2 {
                if self.state == false {
                    self.state = true;
                    return true;
                }
            } else {
                if self.state == true {
                    self.state = false;
                    return true;
                }
            }
        }

        false
    }

    /*
    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {


        true
    }*/

    fn touch_up(&mut self, _x: f32, _y: f32, _context: &mut Context) -> bool {
        false
    }
}