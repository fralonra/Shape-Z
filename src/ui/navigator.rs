
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};
use crate::prelude::*;

pub struct Navigator {

    rect                    : Rect,

    buffer                  : Vec<u8>,
    dirty                   : bool,

    tile_size               : f32,

    hover_pos               : Option<(i32, i32)>,

    offset                  : (f32, f32),

    td_pos                  : (f32, f32),
    td_offset               : (f32, f32),
}

impl Navigator {

    pub fn new() -> Self {

        let tile_size = 70.0;

        Self {
            rect            : Rect::empty(),

            buffer          : vec![],
            dirty           : true,
            tile_size,

            hover_pos       : None,

            offset          : (0.0, 0.0),

            td_pos          : (0.0, 0.0),
            td_offset       : (0.0, 0.0),
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    /// Draw a white circle accelerated via rayon for multithreading
    pub fn draw(&mut self, pixels: &mut [u8], context: &mut Context, world: &World, ctx: &TheContext) {

        if self.buffer.len() != self.rect.width * self.rect.height * 4 {
            self.buffer = vec![0; self.rect.width * self.rect.height * 4];
            self.dirty = true;
        }

        if self.dirty {
            const LINES: usize = 1;
            let width = self.rect.width;
            let height = self.rect.height as f32;

            //let start = self.get_time();

            let start_x = self.rect.width as f32 / 2.0 + self.offset.0;
            let start_y = height / 2.0 + self.offset.1;

            let tile_size = self.tile_size;

            self.buffer
                .par_rchunks_exact_mut(width * LINES * 4)
                .enumerate()
                .for_each(|(j, line)| {
                    for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                        let i = j * width * LINES + i;

                        let x = (i % width) as f32;
                        let y = height - (i / width) as f32;

                        let mut color = [0.075, 0.075, 0.075, 1.0];

                        let tile_half_x = tile_size / 2.0;
                        let tile_half_y = tile_size / 4.0;

                        let offset_from_0_x = x - start_x;
                        let offset_from_0_y = y - start_y;

                        let grid_x = offset_from_0_x / tile_half_x;
                        let grid_y = offset_from_0_y / tile_half_y;

                        let grid_x_screen = grid_x * tile_half_x;
                        let grid_y_screen = grid_y * tile_half_y;

                        //let grid_x_screen_start = start_x - tile_half_x * grid_x.ceil();//(grid_x.floor() - 1.0) * tile_half_x;
                        //let grid_y_screen_start = start_y - tile_half_y * grid_y.ceil();//(grid_y.floor() - 1.0) * tile_half_y;

                        let map_x;
                        let map_y;

                        // if self.context.curr_perspective == Perspective::Iso {
                        //     map_x = ((grid_x_screen / tile_half_x + grid_y_screen / tile_half_y) / 2.0) as f32;
                        //     map_y = ((grid_y_screen / tile_half_y -(grid_x_screen / tile_half_x)) / 2.0) as f32;
                        // } else {
                            map_x = grid_x_screen / tile_size * 2.0;
                            map_y = grid_y_screen / tile_size * 2.0;
                        // }

                        // Grid

                        #[inline(always)]
                        pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
                            let t = ((x - e0) / (e1 - e0)). clamp(0.0, 1.0);
                            return t * t * (3.0 - 2.0 * t);
                        }

                        #[inline(always)]
                        pub fn mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                            [   (1.0 - v) * a[0] + b[0] * v,
                                (1.0 - v) * a[1] + b[1] * v,
                                (1.0 - v) * a[2] + b[2] * v,
                                (1.0 - v) * a[3] + b[3] * v ]
                        }

                        let c_x = (map_x * std::f32::consts::PI * 2.0).cos();
                        let c_y = (map_y * std::f32::consts::PI * 2.0).cos();

                        let mut v = smoothstep(0.99, 1.0, c_x.max(c_y));

                        // Handle hover
                        // if let Some(hover_pos) = &self.hover_pos {
                        //     if &(map_x.floor() as i32, map_y.floor() as i32) == hover_pos {
                        //         v = 1.0;
                        //     }
                        // }

                        let key = (map_x.floor() as i32, map_y.floor() as i32, 0_i32);
                        if context.curr_key == Some(vec3i(key.0, key.1, key.2)) {
                            v = 1.0;
                        } else
                        if world.tiles.contains_key(&key) {
                            v = 0.6;
                        }

                        let highlight = [0.3, 0.3, 0.3, 1.0];

                        color = mix_color(&color, &highlight, v);

                        // Search tiles
                        /*
                        for pos in &self.render.sorted_tiles {

                            if let Some(tile) = self.render.tiles.get(&pos) {
                                let xx;
                                let yy;

                                if self.context.curr_perspective == Perspective::Iso {
                                    xx = (pos.0 - pos.1) as F * tile_size / 2.0;
                                    yy = (pos.0 + pos.1) as F * tile_size / 4.0;
                                } else {
                                    xx = pos.0 as F * tile_size / 2.0 + tile_half_x;
                                    yy = pos.1 as F * tile_size / 2.0;
                                }

                                let tile_x = start_x + xx - tile_size / 2.0;
                                let tile_y = start_y + yy - tile_size / 2.0;

                                if x > tile_x && y > tile_y && x < tile_x + tile_size && y < tile_y + tile_size {
                                    let dx = (x - tile_x) as usize;
                                    let dy = (y - tile_y) as usize;

                                    let off = dx * 4 + dy * tile_size as usize * 4;

                                    let alpha = tile.pixels[off + 3];
                                    if alpha > 0.0 {
                                        let c =[tile.pixels[off], tile.pixels[off+1], tile.pixels[off+2], alpha];
                                        color = mix_color(&color, &c, alpha);
                                    }
                                }
                            }
                        }*/

                        pixel.copy_from_slice(&[(color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8, (color[3] * 255.0) as u8]);
                    }
                });
            // let stop = self.get_time();
            // println!("tick time {:?}", stop - start);
            self.dirty = false;
        }
        ctx.draw.copy_slice(pixels, &self.buffer, &self.rect.to_usize(), ctx.width);
    }

    /// Click / touch at the given position, check if we clicked inside the circle and if yes initialize dragging
    pub fn touch_down(&mut self, x: f32, y: f32, context: &mut Context) -> bool {

        self.td_pos = (x, y);
        self.td_offset = self.offset.clone();

        let p = self.grid_pos_for_screen_coord(x, y);

        //println!("{:?}", p);

        context.cmd = Some(Command::TileSelected(p.0, 0, p.1));
        self.dirty = true;
        true
    }

    pub fn touch_dragged(&mut self, x: f32, y: f32) -> bool {
        self.offset.0 = self.td_offset.0 - (self.td_pos.0 - x);
        self.offset.1 = self.td_offset.1 - (self.td_pos.1 - y);

        self.dirty = true;
        true
    }

    /*
    pub fn touch_up(&mut self, x: f32, y: f32) -> bool {
        false
    }*/

    pub fn hover(&mut self, x: f32, y: f32) -> bool {
        let new_pos = self.grid_pos_for_screen_coord(x, y);
        if Some(new_pos) != self.hover_pos {
            self.hover_pos = Some(self.grid_pos_for_screen_coord(x, y));
            return true;
        }

        false
    }

    /// Returns the grid position for the given screen coordinate
    fn grid_pos_for_screen_coord(&self, x: f32, y: f32) -> (i32, i32) {

        let start_x = self.rect.width as f32 / 2.0 + self.offset.0;
        let start_y = self.rect.height as f32 / 2.0 + self.offset.1;

        let offset_from_0_x = (x - self.rect.x as f32) - start_x;
        let offset_from_0_y = (y - self.rect.y as f32) - start_y;

        // if self.context.curr_perspective == Perspective::Iso {
        //     let tile_half_x = self.tile_size / 2.0;
        //     let tile_half_y = self.tile_size / 4.0;

        //     let grid_x = (offset_from_0_x) / tile_half_x;
        //     let grid_y = (offset_from_0_y) / tile_half_y;

        //     let grid_x_screen = grid_x * tile_half_x;
        //     let grid_y_screen = grid_y * tile_half_y;

        //     let map_x = ((grid_x_screen / tile_half_x + grid_y_screen / tile_half_y) / 2.0) as f32;
        //     let map_y = ((grid_y_screen / tile_half_y -(grid_x_screen / tile_half_x)) / 2.0) as f32;

        //     let map_grid_x = map_x.floor() as i32;
        //     let map_grid_y = map_y.floor() as i32;

        //     //println!("{:?}", (map_grid_x, map_grid_y));
        //     (map_grid_x, map_grid_y)
        // } else {
            let tile_half_x = self.tile_size / 2.0;
            let tile_half_y = self.tile_size / 4.0;

            let grid_x = (offset_from_0_x) / tile_half_x;
            let grid_y = (offset_from_0_y) / tile_half_y;

            let grid_x_screen = grid_x * tile_half_x;
            let grid_y_screen = grid_y * tile_half_y;

            let map_x = grid_x_screen / self.tile_size * 2.0;
            let map_y = grid_y_screen / self.tile_size * 2.0;

            let map_grid_x = map_x.floor() as i32;
            let map_grid_y = map_y.floor() as i32;

            //println!("{:?}", (map_grid_x, map_grid_y));
            (map_grid_x, map_grid_y)
        // }
    }

    /// Gets the current time in milliseconds
    fn _get_time(&self) -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }

}