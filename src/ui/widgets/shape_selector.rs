use crate::prelude::*;

pub enum ShapeSelectorMode {
    SDF
}

pub struct ShapeSelector {
    rect                        : Rect,

    sdf_previews                : (Vec<u8>, Rect),
    sdf_index                   : Option<usize>
}

impl Widget for ShapeSelector {

    fn new() -> Self {

        let sdf_previews = create_shape_previews();

        Self {
            rect                : Rect::empty(),

            sdf_previews,
            sdf_index           : None,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, _world: &World, ctx: &TheContext) {

        let r = self.rect.to_usize();

        let prev_rect = Rect::new(r.0, r.1, self.sdf_previews.1.width, self.sdf_previews.1.height);
        ctx.draw.rect(pixels, &r, ctx.width, &context.color_toolbar);

        ctx.draw.copy_slice(pixels, &self.sdf_previews.0, &prev_rect.to_usize(), ctx.width);
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

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, _world: &World) -> bool {
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