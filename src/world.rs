use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};
use rand::{thread_rng, Rng, rngs::ThreadRng};

#[derive(Clone)]
pub struct World {
    pub camera              : Camera,

    pub tiles               : FxHashMap<(i32, i32, i32), Tile>,
    pub needs_update        : bool,

    pub curr_tool           : Tool,
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

            curr_tool       : Tool::new("".into()),
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

    pub fn render(&self, buffer: &mut ColorBuffer, context: &Context, iteration: i32) {

        let width = buffer.width;
        let height = buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;

        let start = self.get_time();

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {
                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let mut rng: ThreadRng = thread_rng();

                    // let cam_off = hash3_2(vec3f(time, uv.x, uv.y));
                    let cam_off = vec2f(rng.gen(), rng.gen());
                    //let cam_off = vec2f(0.5, 0.5);
                    let ray = self.camera.create_ray(uv, screen, cam_off);

                    let mut color = [uv.x, uv.y, 0.0, 1.0];

                    if let Some(hit) = self.dda_recursive(&ray) {
                        //color = [hit.normal.x.abs(), hit.normal.y.abs(), hit.normal.z.abs(), 1.0];
                        color = context.palette.at_f_to_linear(hit.value);

                        // Ambient occlusion
                        let pos = hit.hitpoint - 0.01 * hit.normal;

                        let z = hit.normal;
                        let x = normalize(cross(z, vec3f(-0.36, -0.48, 0.8)));
                        let y = normalize(cross(z, x));

                        // let hash = hash3_2(vec3f(time + 0.2, uv.x, uv.y));
                        let hash = vec2f(rng.gen(), rng.gen());
                        let mut a = sqrt(hash.x);
                        let b = a * cos(6.283185 * hash.y);
                        let c = a * sin(6.283185 * hash.y);
                        a = sqrt(1.0 - hash.x);
                        let shade_dir = b * x + c * y + a * z;

                        let ambient;
                        if let Some(_) = self.dda_recursive(&Ray::new(pos, shade_dir)) {
                            ambient = 0.0;
                        } else {
                            ambient = 1.0;
                        }

                        // Sun
                        let mut z = vec3f(0.48, 0.36, 0.8);
                        let x = normalize(cross(z, vec3f(0.0, 1.0, 0.0)));
                        let y = normalize(cross(z, x));

                        // let hash = hash3_2(vec3f(time + 0.3, uv.x, uv.y));
                        let hash = vec2f(rng.gen(), rng.gen());
                        //let hash = vec2f(0.5, 0.5);
                        let a = sqrt(hash.x);
                        let b = a * cos(6.283185 * hash.y);
                        let c = a * sin(6.283185 * hash.y);
                        z += 0.04 * (b * x + c * y);

                        let sun;
                        if let Some(_) = self.dda_recursive(&Ray::new(pos, normalize(z))) {
                            sun = 0.0;
                        } else {
                            sun = 1.0;
                        }

                        color[0] *= 0.6 * ambient + 0.4 * sun;
                        color[1] *= 0.6 * ambient + 0.4 * sun;
                        color[2] *= 0.6 * ambient + 0.4 * sun;

                        // Clip color to the palette
                        let index = context.palette.closest(color[0], color[1], color[2]);
                        color = context.palette.at_f(index);
                    }

                    #[inline(always)]
                    pub fn mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    // Accumulate
                    let mix = mix_color(pixel, &color, 1.0 / (iteration + 1) as f32);
                    pixel.copy_from_slice(&mix);
                }
        });

        let stop = self.get_time();
        println!("tick time {:?}", stop - start);

    }

    pub fn hit_at(&self, pos: Vec2f, buffer: &ColorBuffer) -> Option<HitRecord> {

        let x: f32 = pos.x / buffer.width as f32;
        let y: f32 = pos.y / buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        let uv = vec2f(x, 1.0 - y);

        let ray = self.camera.create_ray(uv, screen, vec2f(0.5, 0.5));

        if let Some(hit) = self.dda_recursive(&ray) {
            Some(hit)
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

    fn dda_recursive(&self, ray: &Ray) -> Option<HitRecord> {

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

        let mut normal;//= Vec3f::zero();
        let srd = signum(rd);

        let rdi = 1.0 / (2.0 * rd);

        let mut key: Vec3<i32>;// = Vec3i::zero();

        for _ii in 0..20 {
            key = Vec3i::from(i);

            if let Some(tile) = self.tiles.get(&(key.x, key.y, key.z)) {

                let mut lro = ray.at(dist);
                lro -= Vec3f::from(key);
                lro *= tile.size as f32;
                //lro = lro - rd * 1.0;

                if let Some(mut hit) = tile.dda(&Ray::new(lro, rd)) {
                    hit.key = key;
                    hit.hitpoint = ray.at(dist + hit.distance / (tile.size as f32));
                    hit.distance = dist;
                    return Some(hit);
                }
            }

            let plain = (1.0 + srd - 2.0 * (ro - i)) * rdi;
            dist = min(plain.x, min(plain.y, plain.z));
            normal = equal(dist, plain) * srd;
            i += normal;
        }

        None
        /*
        if hit {
            let mut hit_record = HitRecord::new();

            hit_record.hitpoint = ray.at(dist);
            hit_record.key = key;
            hit_record.distance = dist;
            hit_record.normal = normal;

            Some(hit_record)
        } else {
            None
        }*/
    }

    /// Gets the current time in milliseconds
    fn get_time(&self) -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let stop = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            stop.as_millis()
    }

}