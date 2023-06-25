
use crate::prelude::*;

pub struct ExtrusionTool {
    rect                    : Rect,

    shape                   : Shape,
}

impl Widget for ExtrusionTool {

    fn new() -> Self {

        Self {
            rect        : Rect::empty(),

            shape       : Shape::new()
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn draw(&mut self, pixels: &mut [u8], context: &mut Context, _world: &World, ctx: &TheContext) {

        let stride = self.rect.width;
        let shape_rect = Rect::new(self.rect.x, self.rect.y, self.rect.width, self.rect.width);
        self.shape.draw(pixels, shape_rect, stride, context, ctx);

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

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, _world: &World) -> bool {

        /*
        if self.rect.is_inside((x as usize, y as usize)) {
            context.cmd = self.cmd.clone();
            if self.has_state {
                self.state = !self.state;
            } else {
                self.clicked = true;
            }
            return true;
        }*/

        false
    }

    /*
    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {


        true
    }*/

    fn touch_up(&mut self, _x: f32, _y: f32, _context: &mut Context) -> bool {

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

                            let h = 0.3;

                            pos.y -= h;
                            let p = pos.xz() + vec2f(0.5, 0.5) - hp.xz();

                            // Get the distance
                            let (mut d, _index) = self.shape.distance(p);

                            // Annular
                            d = abs(d) - 0.05;

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