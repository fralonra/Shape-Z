
use crate::prelude::*;

pub struct TileEditor {
    rect                        : Rect,

    voxels_r                    : Rect,
    palette_r                   : Rect,

    cam_orbit_drag              : Option<(f32, f32, f32, f32)>,

    buffer                      : ColorBuffer,
    widgets                     : Vec<Box<dyn Widget>>,

    pub tile_needs_update       : bool,
}

impl Widget for TileEditor {

    fn new() -> Self {
        let mut widgets : Vec<Box<dyn Widget>> = vec![];

        /*
        let mut clear_button = Box::new(TextButton::new());
        clear_button.set_text("Clear".into());
        clear_button.set_cmd(Command::ClearProperty);

        let mut shape_button = Box::new(TextButton::new());
        shape_button.set_text("Shape".into());
        shape_button.set_has_state(true);
        shape_button.set_cmd(Command::ShapeProperty);

        let mut profile_button = Box::new(TextButton::new());
        profile_button.set_text("Profile".into());
        profile_button.set_has_state(false);
        profile_button.set_cmd(Command::ProfileProperty);

        let mut color_front_button = Box::new(TextButton::new());
        color_front_button.set_text("Front".into());
        color_front_button.set_has_state(false);
        color_front_button.set_cmd(Command::ColorFrontProperty);

        let mut color_side_button = Box::new(TextButton::new());
        color_side_button.set_text("Side".into());
        color_side_button.set_has_state(false);
        color_side_button.set_cmd(Command::ColorSideProperty);

        let mut color_top_button = Box::new(TextButton::new());
        color_top_button.set_text("Top".into());
        color_top_button.set_has_state(false);
        color_top_button.set_cmd(Command::ColorTopProperty);

        widgets.push(clear_button);
        widgets.push(shape_button);
        widgets.push(profile_button);
        widgets.push(color_front_button);
        widgets.push(color_side_button);
        widgets.push(color_top_button);
        */

        Self {
            rect                : Rect::empty(),

            voxels_r            : Rect::empty(),
            palette_r           : Rect::empty(),

            cam_orbit_drag      : None,

            buffer              : ColorBuffer::new(280, 280),

            widgets,

            tile_needs_update   : false,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        let r = self.rect.to_usize();
        ctx.draw.rect(pixels, &r, ctx.width, &context.color_widget);

        if context.curr_key.is_none() { return; }

        let tile_size = r.2 - 20;

        if tile_size != self.buffer.width || tile_size != self.buffer.height {
            self.buffer = ColorBuffer::new(tile_size, tile_size);
            self.tile_needs_update = true;
        }

        if self.tile_needs_update {
            context.curr_tile.render(&mut self.buffer);
            self.tile_needs_update = false;
        }
        self.buffer.convert_to_u8_at(pixels, (r.0 + 10, r.1 + 10, ctx.width, ctx.height));
        self.voxels_r = Rect::new(r.0 + 10, r.1 + 10, tile_size, tile_size);
        /*
        // Property

        let mut pr = (x, y, pixel_size, pixel_size);

        for y in 0..property_dim {
            for x in 0..property_dim {

                let v = context.curr_property().get(x, y);

                let color: [u8; 4] = if v == 0 { context.color_selected } else {
                    if context.curr_property == Props::Shape {
                        context.color_text
                    } else {
                        context.palette.palette[v as usize]
                    }
                };

                context.draw2d.draw_rect(pixels, &pr, context.width, &color);

                pr.0 += pixel_size + 1;
            }
            pr.0 = x;
            pr.1 += pixel_size + 1;
        }

        self.prop_r = Rect::new(x as u32, y as u32, available_width as u32, available_width as u32);
        self.prop_pixel_size = pixel_size + 1;

        // Widgets

        self.widgets[0].set_rect(Rect::new(self.rect.x + 10, self.rect.y + 240, 70, 20));

        self.widgets[1].set_rect(Rect::new(self.rect.x + 10, self.rect.y + 275, 70, 20));
        self.widgets[2].set_rect(Rect::new(self.rect.x + 10 + 78, self.rect.y + 275, 70, 20));
        self.widgets[3].set_rect(Rect::new(self.rect.x + 10, self.rect.y + 275 + 25, 70, 20));
        self.widgets[4].set_rect(Rect::new(self.rect.x + 10 + 78, self.rect.y + 275 + 25, 70, 20));
        self.widgets[5].set_rect(Rect::new(self.rect.x + 10 + 78 * 2, self.rect.y + 275 + 25, 70, 20));

        for w in &mut self.widgets {
            w.draw(pixels, context, render);
        }

        // Palette

        if context.is_color_property() {

            let x = r.0 + 10;
            let y = r.1 + 330;
            let size = 20;
            let mut pr = (x, y, size, size);

            for index in 0..context.palette.palette.len() {

                let color = context.palette.palette[index];
                context.draw2d.draw_rect(pixels, &pr, context.width, &color);

                if index == context.curr_color_index {
                    if index < 2 {
                        context.draw2d.draw_rect_outline(pixels, &pr, context.width, [255, 255, 255, 255]);
                    } else {
                        context.draw2d.draw_rect_outline(pixels, &pr, context.width, [0, 0, 0, 255]);
                    }
                }

                if index == 10 {
                    pr.0 = x;
                    pr.1 += size + 1;
                } else {
                    pr.0 += size + 1;
                }
            }

            self.palette_r = Rect::new(x as u32, y as u32, 10 * 20 + 20 as u32, 40 as u32);
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
        if context.curr_key.is_none() { return false; }

        if self.rect.is_inside((x as usize, y as usize)) {

            if self.voxels_r.is_inside((x as usize, y as usize)) {
                let x = x - self.voxels_r.x as f32;
                let y = y - self.voxels_r.y as f32;

                let key = context.curr_tile.key_at(vec2f(x, y), &self.buffer);
                println!("key {:?}", key);
            }

            self.cam_orbit_drag = Some((x, y, context.curr_tile.camera.azimuth, context.curr_tile.camera.elevation));

            /*
            // Property
            if self.prop_r.is_inside((x as u32, y as u32)) {

                let px = (x as usize - self.prop_r.x as usize) / self.prop_pixel_size;
                let py = (y as usize - self.prop_r.y as usize) / self.prop_pixel_size;

                let is_color: bool = context.is_color_property();
                let index = context.curr_color_index as u8;
                let property = context.curr_property_mut();

                let new_index;
                if is_color {
                    new_index = index;
                } else {
                    new_index = 1;
                }

                let old_index = property.get(px, py);

                if old_index != new_index {
                    property.set(px, py, new_index);
                    context.cmd = Some(Command::PropertyHasBeenUpdated);
                    return true;
                }
            }

            // Check for palette
            if context.is_color_property() {

                if self.palette_r.is_inside((x as u32, y as u32)) {
                    let px = (x as usize - self.palette_r.x as usize) / 21;
                    let py = (y as usize - self.palette_r.y as usize) / 21;

                    let index = px + py * 10 + py;

                    if index <= 16 {
                        context.curr_color_index = index;
                    }
                }
            }*/

            for w in &mut self.widgets {
                _ = w.touch_down(x, y, context);
            }
            true
        } else {
            false
        }

    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.rect.is_inside((x as usize, y as usize)) {

            if let Some(cam_drag) = self.cam_orbit_drag {

                context.curr_tile.camera.azimuth = cam_drag.2 + (cam_drag.0 - x);
                context.curr_tile.camera.elevation = cam_drag.1 + (cam_drag.1 - y);

                self.tile_needs_update = true;
            }

            /*
            if self.prop_r.is_inside((x as u32, y as u32)) {

                let px = (x as usize - self.prop_r.x as usize) / self.prop_pixel_size;
                let py = (y as usize - self.prop_r.y as usize) / self.prop_pixel_size;

                let is_color = context.is_color_property();
                let index = context.curr_color_index as u8;
                let property = context.curr_property_mut();

                let new_index;
                if is_color {
                    new_index = index;
                } else {
                    new_index = 1;
                }

                let old_index = property.get(px, py);

                if old_index != new_index {
                    property.set(px, py, new_index);
                    context.cmd = Some(Command::PropertyHasBeenUpdated);
                    return true;
                }
            }*/
            true
        } else {
            false
        }
    }

    fn touch_up(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        self.cam_orbit_drag = None;

        let mut consumed = false;
        for w in &mut self.widgets {
            consumed = w.touch_up(x, y, context);
        }
        consumed
    }

    fn update(&mut self, _context: &mut Context) {
        self.tile_needs_update = true;
    }

}