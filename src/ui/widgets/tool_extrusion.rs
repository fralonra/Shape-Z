
use crate::prelude::*;

pub struct ExtrusionTool {
    rect                    : Rect,

    shape                   : Shape,

    size_widget             : Box<dyn Widget>,
    value_list              : ValueList,

    curr_sdf_index          : Option<usize>,
    sdf_rects               : Vec<Rect>,
}

impl Widget for ExtrusionTool {

    fn new() -> Self {

        let shape = Shape::new();

        let mut size_widget = Box::new(IntSlider::new());
        size_widget.set_text("Length".to_string());
        size_widget.set_range(1, 200);
        size_widget.set_value(shape.size);

        let mut value_list = ValueList::new();

        if let Some(sdf) = &shape.sdf[0] {
            value_list.set_value_list(sdf.parameters.clone());
        }

        Self {
            rect           : Rect::empty(),

            shape,

            size_widget,
            value_list,

            curr_sdf_index  : Some(0),
            sdf_rects       : vec![Rect::empty(); 5]
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], _stride: usize, context: &mut Context, world: &World, ctx: &TheContext) {

        let stride = self.rect.width;
        let shape_width = self.rect.width - 80;
        let shape_rect = Rect::new(self.rect.x, self.rect.y + 22, shape_width, shape_width);
        self.shape.draw(pixels, shape_rect, stride, context, ctx);

        if let Some(font) = &context.font {
            ctx.draw.blend_text_rect(pixels, &(self.rect.x + 10, self.rect.y, shape_width, 20), stride, &font, 15.0, &"EXTRUDE TOOL".to_string(), &context.color_text, theframework::thedraw2d::TheTextAlignment::Left)
        }

        // Size Widget

        let size_rect = Rect::new(self.rect.x + 10, self.rect.y + 200, 200, 24);
        self.size_widget.set_rect(size_rect);
        self.size_widget.draw(pixels, stride, context, world, ctx);

        // SDFs

        let mut sdf_rect = Rect::new(self.rect.x + 10, self.rect.y + 240, 40, 40);
        for (index, s) in self.shape.sdf.iter().enumerate() {
            if let Some(sdf) = s {
                sdf.create_preview(pixels, sdf_rect, stride);
            } else {
                ctx.draw.rounded_rect(pixels, &sdf_rect.to_usize(), stride, &context.color_black, &(6.0, 6.0, 6.0, 6.0));
            }
            if Some(index) == self.curr_sdf_index {
                ctx.draw.rounded_rect_with_border(pixels, &sdf_rect.to_usize(), stride, &[0, 0, 0, 0], &(6.0, 6.0, 6.0, 6.0), &context.color_white, 2.0);
            }

            self.sdf_rects[index] = sdf_rect.clone();
            sdf_rect.x += 45;
        }

        // Modifiers

        //let mut modifier_rect = Rect::new(self.rect.x + 50, self.rect.y + 280, 40, 40);
        /*
        for (index, s) in self.shape.sdf.iter().enumerate() {
            if let Some(sdf) = s {
                sdf.create_preview(pixels, modifier_rect, stride);
            } else {
                ctx.draw.rounded_rect(pixels, &modifier_rect.to_usize(), stride, &context.color_black, &(6.0, 6.0, 6.0, 6.0));
            }
            if Some(index) == self.curr_sdf_index {
                ctx.draw.rounded_rect_with_border(pixels, &modifier_rect.to_usize(), stride, &[0, 0, 0, 0], &(6.0, 6.0, 6.0, 6.0), &context.color_white, 2.0);
            }

            self.sdf_rects[index] = modifier_rect.clone();
            modifier_rect.x += 45;
        }*/

        let value_list_rect = Rect::new(self.rect.x + 10, self.rect.y + 300, self.rect.width - 20, 200);
        self.value_list.set_rect(value_list_rect);
        self.value_list.draw(pixels, stride, context, world, ctx);

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

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, world: &World) -> bool {

        let lx = x as usize - self.rect.x;
        let ly = y as usize - self.rect.y;

        if self.rect.is_inside((lx, ly)) {

            if self.size_widget.contains(x, y) {
                if self.size_widget.touch_down(x, y, context, world) {
                    self.shape.size = self.size_widget.get_value();
                    return true;
                }
            }

            // SDF ?
            for (index, s) in self.sdf_rects.iter().enumerate() {
                if s.is_inside((lx, ly)) {
                    self.curr_sdf_index = Some(index);
                    if let Some(sdf) = &self.shape.sdf[index] {
                        self.value_list.set_value_list(sdf.parameters.clone());
                    }
                    return true;
                }
            }

            if self.value_list.contains(x, y) {
                if self.value_list.touch_down(x, y, context, world) {
                    let list = self.value_list.generate_value_list();
                    if let Some(curr_sdf_index) = self.curr_sdf_index {
                        if let Some(sdf) = &mut self.shape.sdf[curr_sdf_index] {
                            sdf.parameters = list;
                        }
                    }
                    return true;
                }
            }
        }

        false
    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.size_widget.contains(x, y) {

            if self.size_widget.touch_dragged(x, y, context) {
                self.shape.size = self.size_widget.get_value();
                return true;
            }
        }

        if self.value_list.contains(x, y) {
            if self.value_list.touch_dragged(x, y, context) {
                let list = self.value_list.generate_value_list();
                if let Some(curr_sdf_index) = self.curr_sdf_index {
                    if let Some(sdf) = &mut self.shape.sdf[curr_sdf_index] {
                        sdf.parameters = list;
                    }
                }
                return true;
            }
        }

        false
    }

    fn touch_up(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        if self.size_widget.touch_up(x, y, context) {
            return true;
        }

        false
    }

    // Tool

    /// Set the shape
    fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    fn hit(&mut self, hit: HitRecord, tiles: &Vec<Vec3i>) {

        let mut world = WORLD.lock().unwrap();

        let hp = world.to_world_coord(hit.key, hit.tile_key);

        for tile_key in tiles {
            if let Some(mut tile) = world.get_tile(*tile_key) {

                let size = tile.size;

                for y in 0..size {
                    for x in 0..size {
                        for z in 0..size {
                            let mut pos = world.to_world_coord(*tile_key, vec3i(x as i32, y as i32, z as i32));

                            let h = self.shape.size as f32 / 100.0;

                            pos.y -= h;
                            let p = pos.xz() + vec2f(0.5, 0.5) - hp.xz();

                            // Get the distance
                            let (mut d, _index) = self.shape.distance(p);

                            // Annular
                            //d = abs(d) - 0.05;

                            // Extrude
                            let w = vec2f( d, abs(pos.y - hp.y) - h );
                            d = min(max(w.x,w.y),0.0) + length(max(w,vec2f(0.0, 0.0)));

                            if d < 0.0 {
                                tile.set_voxel(x, y, z, Some((10, 10)));
                            }
                        }
                    }
                }

                world.set_tile(*tile_key, tile);
            }
        }

        world.needs_update = true;
    }

    fn sdf_triggered(&mut self, sdf: SDFType) {
        if let Some(curr_sdf_index) = self.curr_sdf_index {
            self.shape.sdf[curr_sdf_index] = Some(SDF::new(sdf));
            if let Some(sdf) = &self.shape.sdf[curr_sdf_index] {
                self.value_list.set_value_list(sdf.parameters.clone());
            }
        }
    }

}

/*
pub trait Extrusion {
    fn set_shape(&mut self, shape: Shape);
}

impl Extrusion for ExtrusionTool {
    fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }
}*/