
use crate::prelude::*;

pub struct Browser {
    rect                : Rect,

    content_rects       : Vec<Rect>,
    ids                 : Vec<Uuid>,
}

impl Widget for Browser {

    fn new() -> Self {

        Self {
            rect            : Rect::empty(),
            content_rects   : vec![],
            ids             : vec![],
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        let mut r = self.rect.to_usize();
        ctx.draw.blend_rect(pixels, &r, context.width, &context.color_widget);
        ctx.draw.rect(pixels, &(r.0 + r.2, r.1, 1, r.3), ctx.width, &[0, 0, 0, 255]);

        r.0 += 40;
        r.1 += 10;
        r.2 = 100;
        r.3 = 100;

        self.content_rects = vec![];
        self.ids = vec![];

        /*
        if context.is_color_property() {
            for (index, pattern) in context.patterns.iter().enumerate() {

                self.content_rects.push(Rect::new(r.0 as u32, r.1 as u32, 100, 100));

                let color = if index == context.curr_pattern { context.color_selected } else { context.color_widget };

                context.draw2d.draw_rounded_rect(pixels, &r, context.width, &color, &(5.0, 5.0, 5.0, 5.0));

                let mut pattern_pixels = vec![0; 100 * 100 * 4];

                self.render.render_pattern_preview(100, &mut pattern_pixels, pattern);
                context.draw2d.blend_slice(pixels, &mut pattern_pixels[..], &r, context.width);

                r.0 += 120;
            }
        } else
        if context.curr_property == Props::Shape {

            let mut preview = vec![0; 100 * 100 * 4];

            if let Some(pos) = context.selected_pos {

                if let Some(tile) = render.tiles.get(&pos) {
                    for (index, shape) in tile.shapes.iter().enumerate() {

                        self.content_rects.push(Rect::new(r.0 as u32, r.1 as u32, 100, 100));

                        let color = if Some(shape.id()) == context.selected_id { context.color_selected } else { context.color_widget };

                        context.draw2d.draw_rounded_rect(pixels, &r, context.width, &color, &(5.0, 5.0, 5.0, 5.0));

                        tile.render_shape_preview(&context.palette, index, 100, &mut preview);

                        context.draw2d.blend_slice(pixels, &preview, &r, context.width);

                        self.ids.push(shape.id());

                        r.0 += 120;
                    }
                }
            }
        }*/
    }

    fn contains(&mut self, x: f32, y: f32) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {
            true
        } else {
            false
        }
    }

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.contains(x, y) {

            /*
            if context.is_color_property(){
                for (index, r) in self.content_rects.iter().enumerate() {
                    if r.is_inside((x as u32, y as u32)) {
                        context.curr_pattern = index;
                        context.cmd = Some(Command::InsertPattern);
                    }
                }
            } else
            if context.curr_property == Props::Shape {
                for (index, r) in self.content_rects.iter().enumerate() {
                    if r.is_inside((x as u32, y as u32)) {
                        context.curr_shape = index;

                        context.selected_id = Some(self.ids[index]);
                        context.cmd = Some(Command::CopySelectedShapeProperties);
                    }
                }
            }*/

            true
        } else {
            false
        }
    }
    /*
    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {


        true
    }

    fn touch_up(&mut self, _x: f32, _y: f32, context: &mut Context) -> bool {
        false
    }*/
}