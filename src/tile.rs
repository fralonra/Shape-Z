use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

#[derive(PartialEq, Debug, Clone)]
pub struct Tile {
    pub camera              : Camera
}

impl Tile {
    pub fn new() -> Self {
        Self {
            camera          : Camera::new(vec3f(0.0, 0.0, 5.0), Vec3f::zero(), 70.0),
        }
    }

    pub fn render(&self, buffer: &mut ColorBuffer) {

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

                    let ray = self.camera.create_ray(uv, screen);

                    let mut color = [uv.x, uv.y, 0.0, 1.0];

                    if let Some(hit) = self.dda(&ray) {
                        color = [hit.normal.x.abs(), hit.normal.y.abs(), hit.normal.z.abs(), 1.0];
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

            //println!("{:?}", i);

        for _ii in 0..20 {

            let id: Vec3<i32> = Vec3i::from(i);
            if id.x == -2 && id.y == 0 && id.z == 0 {
                hit = true;
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

            Some(hit_record)
        } else {
            None
        }
    }
}