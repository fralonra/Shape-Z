
use crate::prelude::*;

pub struct IntSlider {
    rect                : Rect,

    text                : String,

    min                 : i32,
    max                 : i32,

    value               : i32,

    clicked             : bool,
}

impl Widget for IntSlider {

    fn new() -> Self {

        Self {
            rect        : Rect::empty(),
            text        : "".to_string(),

            min         : 0,
            max         : 0,

            value       : 0,

            clicked     : false,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn set_range(&mut self, min: i32, max: i32) {
        self.min = min;
        self.max = max;
    }

    fn get_value(&mut self) -> i32{
        self.value
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    fn draw(&mut self, pixels: &mut [u8], stride: usize, context: &mut Context, _world: &World, ctx: &TheContext) {

        let rounding = (self.rect.height as f32 / 2.0 + 0.5).ceil();

        let v = self.value as f32;

        ctx.draw.rounded_rect(pixels, &self.rect.to_usize(), stride, &context.color_button, &(rounding, rounding, rounding, rounding));

        let min = self.min as f32;

        if v > min {
            let max = self.max as f32;
            let pp = self.rect.width as f32 / (max - min);

            let mut r = self.rect.to_usize();
            let left_off = ((v - 1.0) * pp).round() as usize;

            if left_off < r.2 {
                r.2 = left_off;
                let mut round = (rounding, rounding, rounding, rounding);
                if v < max {
                    round.0 = 0.0;
                    round.1 = 0.0;
                } else {
                    r.2 = self.rect.width;
                }

                ctx.draw.rounded_rect(pixels, &r, stride, &context.color_selected, &round);

            }

            if let Some(font) = &context.font {
                ctx.draw.blend_text_rect(pixels, &self.rect.to_usize(), stride, &font, 16.0, &format!("{}{}", self.text, v as i32), &context.color_text, theframework::thedraw2d::TheTextAlignment::Center);
            }
        }
        /*
        let color: [u8; 4] = if self.clicked || self.state { context.color_selected } else { context.color_button };

        let r = self.rect.to_usize();
        ctx.draw.rounded_rect(pixels, &r, context.width, &color, &(6.0, 6.0, 6.0, 6.0));

        if let Some(font) = &context.font {
            ctx.draw.blend_text_rect(pixels, &r, context.width, &font, 16.0, &self.text, &context.color_text, theframework::thedraw2d::TheTextAlignment::Center)
        }*/
    }

    fn contains(&mut self, x: f32, y: f32) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {
            true
        } else {
            false
        }
    }

    fn touch_down(&mut self, x: f32, y: f32, _context: &mut Context, _world: &World) -> bool {

        if self.rect.is_inside((x as usize, y as usize)) {

            let min = self.min as f32;
            let max = self.max as f32;

            if  x >= self.rect.x as f32 {
                let offset = x - self.rect.x as f32;

                let pp = (max - min) / self.rect.width as f32;
                let v = (min + offset * pp).round().clamp(min, max);

                self.value = v.round() as i32;
            }

            self.clicked = true;
            return true;
        }

        false
    }

    fn touch_dragged(&mut self, x: f32, _y: f32, _context: &mut Context) -> bool {

        if self.clicked {

            let min = self.min as f32;
            let max = self.max as f32;

            if  x >= self.rect.x as f32 {

                if x > self.rect.x as f32 + self.rect.width as f32 {
                    self.value = self.max;
                } else {
                    let offset = x - self.rect.x as f32;

                    let pp = (max - min) / self.rect.width as f32;
                    let v = (min + offset * pp).round().clamp(min, max);

                    self.value = v.round() as i32;
                }
            }
            return true;
        }

        false
    }

    fn touch_up(&mut self, _x: f32, _y: f32, _context: &mut Context) -> bool {
        if self.clicked {
            self.clicked = false;
            return true;
        }
        false
    }
}