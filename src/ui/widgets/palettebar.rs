
use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
enum Mode {
    Color,
    Material
}

pub struct PaletteBar {

    mode                : Mode,

    rect                : Rect,
    palette_r           : Rect,

    switch_y            : usize,

    preview             : MaterialPreview,
    preview_buffer      : ColorBuffer
}

impl Widget for PaletteBar {

    fn new() -> Self {

        Self {
            mode        : Mode::Color,

            rect        : Rect::empty(),
            palette_r   : Rect::empty(),

            switch_y    : 0,

            preview     : MaterialPreview::new(),
            preview_buffer : ColorBuffer::new(10, 10),
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, _world: &World, ctx: &TheContext) {

        let r: (usize, usize, usize, usize) = self.rect.to_usize();

        ctx.draw.rect(pixels, &r, ctx.width, &context.color_toolbar);
        ctx.draw.rect(pixels, &(r.0, r.1, r.2, 2), ctx.width, &[0, 0, 0, 255]);
        ctx.draw.rect(pixels, &(r.0, r.1 + r.3 - 1, r.2, 1), ctx.width, &[0, 0, 0, 255]);

        if self.preview_buffer.width != r.2 || self.preview_buffer.height != r.2 {
            self.preview_buffer = ColorBuffer::new(r.2, r.2);
        }

        let x = r.0 + 0;
        let y = r.1 + 2;
        let size = 16;
        let mut pr = (x, y, size, size);

        // Palette

        if self.mode == Mode::Color {

            let mut in_row = 0;
            let mut row_counter = 0;

            for index in 0..context.palette.colors.len() {

                let color = context.palette.at(index as u8);
                ctx.draw.rect(pixels, &pr, context.width, &color);

                // if index == context.curr_color_index {
                //     if index < 2 {
                //         context.draw2d.draw_rect_outline(pixels, &pr, context.width, [255, 255, 255, 255]);
                //     } else {
                //         context.draw2d.draw_rect_outline(pixels, &pr, context.width, [0, 0, 0, 255]);
                //     }
                // }

                if in_row == 9 {
                    pr.0 = self.rect.x;
                    pr.1 += size;
                    in_row = -1;
                    row_counter += 1;
                } else {
                    pr.0 += size;
                }
                in_row += 1;
            }
            row_counter += 1;
            pr.1 += size;

            self.palette_r = Rect::new(x, y, 10 * size, row_counter * size);
        }

        pr.1 += 2;
        self.switch_y = pr.1;

        // Switch

        let mut br: (usize, usize, usize, usize) = r.clone();
        br.1 = pr.1;
        br.3 = 30;

        //ctx.draw.rounded_rect_with_border(pixels, &br, context.width,  &context.color_toolbar, &(0.0, 0.0, 0.0, 0.0), &context.color_selected, 1.5);

        let mut left_r = br.clone();
        left_r.2 = left_r.2 / 2;

        let mut color = if self.mode == Mode::Color { &context.color_selected } else { &context.color_widget };

        ctx.draw.rect(pixels, &left_r, context.width,  &color);

        let mut right_r = left_r.clone();
        right_r.0 += left_r.2;

        color = if self.mode == Mode::Material { &context.color_selected } else { &context.color_widget };

        ctx.draw.rect(pixels, &right_r, context.width,  &color);

        if let Some(font) = &context.font {
            ctx.draw.blend_text_rect(pixels, &left_r, context.width, &font, 20.0, &"COL".to_string(), &color, theframework::thedraw2d::TheTextAlignment::Center);

            color = if self.mode == Mode::Color { &context.color_selected } else { &context.color_widget };

            ctx.draw.blend_text_rect(pixels, &right_r, context.width, &font, 20.0, &"MAT".to_string(), &color, theframework::thedraw2d::TheTextAlignment::Center);
        }

        //self.preview.render(&mut self.preview_buffer, context);

        self.preview_buffer.convert_to_u8_at(pixels, (0, self.rect.y + self.rect.height - self.rect.width, ctx.width, ctx.height));

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

            if self.palette_r.is_inside((x as usize, y as usize)) {
                let size = 16.0;
                let xx: f32 = (x - self.palette_r.x as f32) / size;
                let yy: f32 = (y - self.palette_r.y as f32) / size;

                let index = (xx.floor() + yy.floor() * 10.0).clamp(0.0, 255.0);

                if self.mode == Mode::Color {
                    self.preview.color = index as u8;
                    context.cmd = Some(Command::ColorIndexChanged(index as u8));
                    self.preview.render(&mut self.preview_buffer, context);
                } else {
                    self.preview.material = index as u8;
                    context.cmd = Some(Command::MaterialIndexChanged(index as u8));
                    self.preview.render(&mut self.preview_buffer, context);
                }

                return true;
            }

            if (y as usize) > self.switch_y && (y as usize) < self.switch_y + 30 {
                if (x as usize) < self.rect.x + self.rect.width / 2 {
                    if self.mode != Mode::Color {
                        self.mode = Mode::Color;
                        return true;
                    }
                } else {
                    if self.mode != Mode::Material {
                        self.mode = Mode::Material;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {
        if self.rect.is_inside((x as usize, y as usize)) {

            if self.palette_r.is_inside((x as usize, y as usize)) {
                let size = 16.0;
                let xx: f32 = (x - self.palette_r.x as f32) / size;
                let yy: f32 = (y - self.palette_r.y as f32) / size;

                let index = (xx.floor() + yy.floor() * 10.0).clamp(0.0, 255.0);
                if self.mode == Mode::Color {
                    self.preview.color = index as u8;
                    context.cmd = Some(Command::ColorIndexChanged(index as u8));
                    self.preview.render(&mut self.preview_buffer, context);
                } else {
                    self.preview.material = index as u8;
                    context.cmd = Some(Command::MaterialIndexChanged(index as u8));
                    self.preview.render(&mut self.preview_buffer, context);
                }

                return true;
            }
        }
        false
    }

    /*
    fn touch_up(&mut self, _x: f32, _y: f32, context: &mut Context) -> bool {
        false
    }*/
}