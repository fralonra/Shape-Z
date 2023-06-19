use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};
use rhai::{ Engine };

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Tile {
    pub camera              : Camera,
    pub size                : usize,

    pub data                : Vec<Option<u8>>,
}

impl Tile {
    pub fn new(size: usize) -> Self {

        let mut camera = Camera::new(vec3f(0.0, 5.0, 5.0), Vec3f::zero(), 70.0);

        let m = (size / 2) as f32 + 0.5;

        camera.center = Vec3f::new(m, m, m);
        camera.origin = camera.center;
        camera.origin.z += 20.0;

        let mut data = vec![None; size * size * size];

        // // Left wall
        for z in 0..size {
            for y in 0..size {
                let index = 0 + y * size + z * size * size;

                data[index] = Some(20);
            }
        }

        // Bottom floor
        for x in 0..size {
            for z in 0..size {
                let index = x + z * size * size;

                data[index] = Some(20);
            }
        }

        Self {
            camera          : camera,

            data            : data,
            size,
        }
    }

    /// Index for a given voxel
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.size + z * self.size * self.size
    }

    /// Get a voxel
    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        if x < self.size && y < self.size && z < self.size {
            self.data[self.index(x, y, z)]
        } else {
            None
        }
    }

    /// Sets a voxel
    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, voxel: Option<u8>)  {
        if x < self.size && y < self.size && z < self.size {
            let index = self.index(x, y, z);
            self.data[index] = voxel;
        }
    }

    /// Resize the content to the new size
    pub fn resize(&mut self, new_size: usize) {
        let mut new_data = vec![None; new_size * new_size * new_size];

        for z in 0..new_size {
            for y in 0..new_size {
                for x in 0..new_size {
                    let old_x = x * self.size / new_size;
                    let old_y = y * self.size / new_size;
                    let old_z = z * self.size / new_size;

                    let old_index = self.index(old_x, old_y, old_z);
                    let new_index = x + y * new_size + z * new_size * new_size;

                    new_data[new_index] = self.data[old_index];
                }
            }
        }

        self.size = new_size;
        self.data = new_data;
    }


    pub fn render(&mut self, buffer: &mut ColorBuffer) {

        let width = buffer.width;
        let height = buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let aa = 1;
                    let mut total = [0.0, 0.0, 0.0, 0.0];

                    for m in 0..aa {
                        for n in 0..aa {

                            let cam_offset = Vec2f::new(m as f32 / aa as f32, n as f32 / aa as f32) - Vec2f::new(0.5, 0.5);

                            let ray = self.camera.create_ray(uv, screen, cam_offset);

                            let mut color = [24.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0, 1.0];

                            if let Some(hit) = self.dda(&ray) {

                                let c1;
                                let c2;

                                if hit.normal.y.abs() > 0.5 {
                                    c1 = (hit.hitpoint.x * std::f32::consts::PI * 2.0).cos();
                                    c2 = (hit.hitpoint.z * std::f32::consts::PI * 2.0).cos();
                                } else
                                if hit.normal.z.abs() > 0.5 {
                                    c1 = (hit.hitpoint.x * std::f32::consts::PI * 2.0).cos();
                                    c2 = (hit.hitpoint.y * std::f32::consts::PI * 2.0).cos();
                                } else {
                                    c1 = (hit.hitpoint.y * std::f32::consts::PI * 2.0).cos();
                                    c2 = (hit.hitpoint.z * std::f32::consts::PI * 2.0).cos();
                                }

                                let v = smoothstep(0.95, 1.0, c1.max(c2));

                                #[inline(always)]
                                pub fn mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                                    [   (1.0 - v) * a[0] + b[0] * v,
                                        (1.0 - v) * a[1] + b[1] * v,
                                        (1.0 - v) * a[2] + b[2] * v,
                                        (1.0 - v) * a[3] + b[3] * v ]
                                }

                                color = [0.0, 0.5, 1.0, 1.0];
                                let highlight: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
                                color = mix_color(&color, &highlight, v);
                            }

                            total[0] += color[0];
                            total[1] += color[1];
                            total[2] += color[2];
                            total[3] += color[3];
                        }

                        let aa_aa = aa as f32 * aa as f32;
                        total[0] /= aa_aa;
                        total[1] /= aa_aa;
                        total[2] /= aa_aa;
                        total[3] /= aa_aa;

                        pixel.copy_from_slice(&total);
                    }
                }
        });
    }

    pub fn key_at(&self, pos: Vec2f, buffer: &ColorBuffer) -> Option<Vec3i> {

        let x: f32 = pos.x / buffer.width as f32;
        let y: f32 = pos.y / buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        let uv = vec2f(x as f32, 1.0 - y);

        let ray = self.camera.create_ray(uv, screen, vec2f(0.5, 0.5));

        if let Some(hit) = self.dda(&ray) {
            println!("{:?}", hit.tile_key);
            Some(hit.tile_key)
        } else {
            None
        }
    }

    pub fn dda(&self, ray: &Ray) -> Option<HitRecord> {

        // Based on https://www.shadertoy.com/view/ct33Rn

        fn equal(l: f32, r: Vec3f) -> Vec3f {
            vec3f(
                if l == r.x { 1.0 } else { 0.0 },
                if l == r.y { 1.0 } else { 0.0 },
                if l == r.z { 1.0 } else { 0.0 },
            )
        }

        let ro = ray.o;
        let rd = ray.d;

        let mut i = floor(ro);

        let mut dist = 0.0;

        let mut normal = Vec3f::zero();
        let srd = signum(rd);

        let rdi = 1.0 / (2.0 * rd);
        let mut hit = false;

        let mut key = Vec3i::zero();
        let mut value : u8 = 0;

        // let bounds_distance = (self.size as f32 - ro) * rdi;

        let max_steps = (self.size as f32 * 3.0).ceil() as i32;

        for _ii in 0..max_steps {

            key = Vec3i::from(i);
            if let Some(voxel) = self.get_voxel(key.x as usize, key.y as usize, key.z as usize) {
                hit = true;
                value = voxel;
                break;
            }

            let plain = (1.0 + srd - 2.0 * (ro - i)) * rdi;
            dist = min(plain.x, min(plain.y, plain.z));

            // if dist > bounds_distance.x && dist > bounds_distance.y && dist > bounds_distance.z {
            //     break;
            // }

            normal = equal(dist, plain) * srd;
            i += normal;
        }

        if hit {
            let mut hit_record = HitRecord::new();

            hit_record.distance = dist;
            hit_record.hitpoint = ray.at(dist);
            hit_record.normal = normal;
            hit_record.value = value;
            hit_record.tile_key = key;

            Some(hit_record)
        } else {
            None
        }
    }

    /// Get the size of the tile
    pub fn get_size(&mut self) -> i32 {
        self.size as i32
    }

    /// Set the voxel at the given position
    pub fn set_voxel_script(&mut self, loc: ScriptVec3i, value: i32) {
        self.set_voxel(loc.v.x as usize, loc.v.y as usize, loc.v.z as usize, Some(value as u8));
    }

    /// Set the voxel at the given position
    pub fn set_voxel_color_script(&mut self, loc: ScriptVec3i, value: String) {
        let mut color_index : Option<u8> = None;

        for v in &WORLD.lock().unwrap().curr_tool.widget_values {
            match v {
                WidgetValue::Color(name, index) => {
                    if *name == value {
                        color_index = Some(*index);
                    }
                }
            }
        }

        if let Some(color_index) = color_index {
            self.set_voxel(loc.v.x as usize, loc.v.y as usize, loc.v.z as usize, Some(color_index as u8));
        }
    }

    /// Clear the voxel at the given position
    pub fn clear_voxel_script(&mut self, loc: ScriptVec3i) {
        self.set_voxel(loc.v.x as usize, loc.v.y as usize, loc.v.z as usize, None);
    }


    /// Register to the engine
    pub fn register(engine: &mut Engine) {
        engine.register_type_with_name::<Tile>("Tile")
            .register_get("size", Tile::get_size)
            .register_fn("set", Tile::set_voxel_script)
            .register_fn("set", Tile::set_voxel_color_script)
            .register_fn("clear", Tile::clear_voxel_script);
    }

}