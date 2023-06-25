use crate::prelude::*;

use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, EnumIter)]
pub enum SDFType {
    Box,
    Circle
}

use SDFType::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SDF {
    sdf_type                    : SDFType,

    pub position                : Vec2f,
    pub size                    : Vec2f,
}

impl SDF {
    pub fn new(sdf_type: SDFType) -> Self {
        Self {
            sdf_type,

            position            : Vec2f::new(0.5, 0.5),
            size                : Vec2f::new(0.4, 0.4),
        }
    }

    pub fn distance(&self, p: Vec2f) -> f32 {
        let mut d = std::f32::MAX;
        if self.sdf_type == Box {
            let q = abs(p - self.position) - self.size;
            d = length(max(q,Vec2f::new(0.0, 0.0))) + min(max(q.x,q.y),0.0);
        } else
        if self.sdf_type == Circle {
            d = length(p - self.position) - self.size.x;
        }
        d
    }

    pub fn apply(&self, key: Vec3i, tile_key: Vec3i) {

        let mut world = WORLD.lock().unwrap();

        if let Some(mut tile) = world.get_tile(key) {

            tile.set_voxel(10, 10, 10, Some((10, 10)));

            let size = tile.size;

            //let start_y = 0;//record.tile_key.y + 1;
            //let height = 1;

            let hp = Vec3f::from(tile_key);

            for y in 0..size {
                for x in 0..size {
                    for z in 0..size {
                        let p = vec3f(x as f32, y as f32, z as f32);
                        // if length(p - hp) - size as f32 / 3.0 < 0.0 {
                        //     //tile.set(vec3i(x, y, z), "Color", "Material");
                        //         tile.set_voxel(x, y, z, Some((10, 10)));
                        // }

                        let p = p - hp; let b = vec3f(10.0, 10.0, 10.0);
                        let q = abs(p) - b;
                        let d = length(max(q,vec3f(0.0, 0.0, 0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
                        if d < 0.0 {
                                tile.set_voxel(x, y, z, Some((10, 10)));
                        }
                    }
                }
            }

            world.set_tile(key, tile);
        }
    }

    pub fn create_preview(&self, pixels: &mut [u8], rect: Rect, stride: usize) {

        let half = rect.width as f32 / 2.0;

        #[inline(always)]
        pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
            (1.0 - v) * a + b * v
        }

        fn shade(d: f32) -> [u8;4] {
            let dist = d*100.0;
            let banding = max(sin(dist), 0.0);
            let strength = sqrt(1.0-exp(-abs(d)*0.2));
            let pattern = mix(&strength, &banding, (0.6-abs(strength-0.5))*0.3);
            let mut c = if d > 0.0 { vec3f(0.0,0.0,0.0) } else { vec3f(0.9,0.9,0.9) };
            c *= pattern;

            [(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8, 255]
        }

        if self.sdf_type == Box {
            let size = half - 5.0;
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    let i = x * 4 + y * stride * 4;

                    let q = abs(vec2f(x as f32, y as f32) - vec2f(half, half)) - vec2f(size, size);
                    let d = length(max(q,Vec2f::new(0.0, 0.0))) + min(max(q.x,q.y),0.0);

                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        } else
        if self.sdf_type == Circle {
            let size = half - 5.0;
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    let i = x * 4 + y * stride * 4;
                    let d = length(vec2f(x as f32, y as f32 - rect.y as f32) - vec2f(half, half)) - size;
                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        }

    }
}

