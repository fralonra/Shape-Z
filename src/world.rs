use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};

#[derive(PartialEq, Debug, Clone)]
pub struct World {
    pub camera              : Camera,

    pub tiles               : FxHashMap<(i32, i32, i32), Tile>,
    pub needs_update        : bool,
}

impl World {
    pub fn new() -> Self {

        let mut tiles  = FxHashMap::default();

        tiles.insert((-1, 0, 0), Tile::new(9));
        tiles.insert((0, 0, 0), Tile::new(9));
        tiles.insert((1, 0, 0), Tile::new(9));

        Self {
            camera          : Camera::new(vec3f(0.0, 5.0, 5.0), Vec3f::zero(), 70.0),

            tiles,
            needs_update    : true,
        }
    }

    /// Get a tile
    pub fn get_tile(&self, at: Vec3i) -> Option<Tile> {
        if let Some(tile) = self.tiles.get(&(at.x, at.y, at.z)) {
            Some(tile.clone())
        } else {
            None
        }
    }

    /// Set a tile
    pub fn set_tile(&mut self, at: Vec3i, tile: Tile) {
        self.tiles.insert((at.x, at.y, at.z), tile);
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

                    let ray = self.camera.create_ray(uv, screen, vec2f(0.5, 0.5));

                    let mut color = [uv.x, uv.y, 0.0, 1.0];

                    if let Some(hit) = self.dda(&ray) {
                        color = [hit.normal.x.abs(), hit.normal.y.abs(), hit.normal.z.abs(), 1.0];
                    }

                    pixel.copy_from_slice(&color);
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
            //println!("{:?}", hit.key);
            Some(hit.key)
        } else {
            None
        }
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

        let mut key: Vec3<i32> = Vec3i::zero();

        for _ii in 0..20 {
            key = Vec3i::from(i);

            if self.tiles.contains_key(&(key.x, key.y, key.z)) {
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

            hit_record.hitpoint = ray.at(dist);
            hit_record.key = key;
            hit_record.distance = dist;
            hit_record.normal = normal;

            Some(hit_record)
        } else {
            None
        }

        /*
        vec3 i = floor(ro);
        float dist;
        int ii;
        vec3 normal;
        vec3 srd = sign(rd);
        vec3 rdi = 1./(2.*rd);
        for (ii = 0; ii < 200; ii += 1) {
            if (getBlock(i)) break;
            vec3 plain = ((1.+srd-2.*(ro-i))*rdi);
            dist = min(plain.x, min(plain.y, plain.z));
            normal = vec3(equal(vec3(dist), plain))*srd;
            i += normal;
        }
        if (ii == 200) return hit(vec3(-1), -1., vec3(-1.0));
        vec3 position = ro+rd*dist;
        return hit(normal, dist, position);*/

    }

}