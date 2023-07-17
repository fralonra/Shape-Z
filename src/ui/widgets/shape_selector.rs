use crate::prelude::*;
use strum::IntoEnumIterator;

#[derive(Clone, PartialEq, Display)]
 enum ShapeSelectorMode {
    SDF
}

pub struct ShapeSelector {
    rect                        : Rect,

    mode                        : ShapeSelectorMode,

    sdf_previews                : (Vec<u8>, Rect),

    curr_index                  : Option<usize>
}

use strum_macros::Display;

impl Widget for ShapeSelector {

    fn new() -> Self {

        let sdf_previews = create_shape_previews();

        Self {
            rect                : Rect::empty(),

            mode                : ShapeSelectorMode::SDF,

            sdf_previews,

            curr_index          : None,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], _stride: usize, context: &mut Context, _world: &World, ctx: &TheContext) {

        let mut r = self.rect.to_usize();

        let prev_rect = Rect::new(r.0, r.1, self.sdf_previews.1.width, self.sdf_previews.1.height);
        ctx.draw.rect(pixels, &r, ctx.width, &context.color_widget);
        ctx.draw.rect(pixels, &(r.0 + r.2 - 1, r.1, 1, r.3), ctx.width, &context.color_black);

        ctx.draw.copy_slice(pixels, &self.sdf_previews.0, &prev_rect.to_usize(), ctx.width);

        if let Some(curr_index) = self.curr_index {
            r.1 += curr_index * 40;
            r.3 = 40;
            ctx.draw.rect_outline(pixels, &r, ctx.width, context.color_white)
        }
        /*
        let color: [u8; 4] = if !self.clicked && !self.state { context.color_selected } else { context.color_button };

        let r = self.rect.to_usize();
        context.draw2d.draw_rounded_rect(pixels, &r, context.width, &color, &(6.0, 6.0, 6.0, 6.0));

        if let Some(font) = &context.font {
            context.draw2d.blend_text_rect(pixels, &r, context.width, &font, 16.0, &self.text, &context.color_text, crate::ui::draw2d::TextAlignment::Center)
        }*/
    }

    fn contains(&mut self, x: f32, y: f32) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {
            true
        } else {
            false
        }
    }

    fn touch_down(&mut self, _x: f32, y: f32, context: &mut Context, _world: &World) -> bool {

        let index = (y as usize - self.rect.y) / 40;

        if self.mode == ShapeSelectorMode::SDF {
            for (sdf_index, sdf) in SDF2DType::iter().enumerate() {
                if sdf_index == index {
                    context.cmd = Some(Command::SDF2DSelected(sdf));
                    self.curr_index = Some(index);
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
        if self.curr_index.is_some() {
            self.curr_index = None;
            return  true;
        }
        false
    }
}