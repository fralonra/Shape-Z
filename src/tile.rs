use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Tile {
    pub camera              : OrbitCamera,
    pub size                : usize,

    pub data                : Vec<Option<u8>>,
}

impl Tile {
    pub fn new(size: usize) -> Self {

        let mut camera = OrbitCamera::new();
        camera.center = Vec3f::new((size / 2) as f32, (size / 2) as f32, (size / 2) as f32);
        camera.origin = camera.center;
        camera.origin.z += 40.0;

        let data = vec![Some(0); size * size * size];

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
        self.camera.update();

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let ray = self.camera.create_ray(uv, screen);

                    let mut color = [0.0, 0.0, 0.0, 1.0];

                    if let Some(hit) = self.dda(&ray) {
                        color = [1.0, 0.0, 0.0, 1.0];
                    }

                    pixel.copy_from_slice(&color);
                }
        });
    }

    fn dda(&self, ray: &Ray) -> Option<HitRecord> {

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

        for _ii in 0..100 {

            key = Vec3i::from(i);
            if let Some(voxel) = self.get_voxel(key.x as usize, key.y as usize, key.z as usize) {
                hit = true;
                value = voxel;
                break;
            }

            let plain = (1.0 + srd - 2.0 * (ro - i)) * rdi;
            dist = min(plain.x, min(plain.y, plain.z));
            normal = equal(dist, plain) * srd;
            i += normal;
        }

        if hit {
            let mut hit_record = HitRecord::new();

            hit_record.distance = dist;
            hit_record.hitpoint = ray.at(dist);
            hit_record.normal = normal;
            hit_record.value = value;

            Some(hit_record)
        } else {
            None
        }
    }
}