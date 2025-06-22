use crate::prelude::*;

use strum_macros::{EnumIter, Display};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, EnumIter, Display)]
pub enum SDF3DType {
    Box,
    Sphere
}

use SDF3DType::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SDF3D {
    sdf_type                    : SDF3DType,

    pub pattern                 : Pattern,

    pub position                : Vec3f,

    pub parameters              : Vec<Value>
}

impl SDF3D {
    pub fn new(sdf_type: SDF3DType) -> Self {

        let parameters= match sdf_type {
            Box => {
                vec![
                    Int("Width".to_string(), 10, 1, 100),
                    Int("Height".to_string(), 10, 1, 100),
                    Int("Depth".to_string(), 10, 1, 100),
                ]
            },
            Sphere => {
                vec![
                    Float("Radius".to_string(), 0.5, 0.001, 10.0),
                ]
            }
        };

        Self {
            sdf_type,

            pattern             : Pattern::new(PatternType::Solid),

            position            : Vec3f::new(1.0, 0.5, 0.5),

            parameters,
        }
    }

    pub fn distance(&self, p: Vec3f) -> f32 {
        let mut d = std::f32::MAX;
        if self.sdf_type == Box {
            /*
            let w = self.parameters[0].get_int() as f32 / 100.0 / 2.0 / zoom;
            let h = self.parameters[1].get_int() as f32 / 100.0 / 2.0 / zoom;
            let depth = self.parameters[2].get_int() as f32 / 100.0 / 2.0 / zoom;

            let q = abs(p - self.position) - vec3f(w, h, depth);
            d = length(max(q,Vec3f::new(0.0, 0.0, 0.0))) + min(max(q.x,q.y),0.0);*/
        } else
        if self.sdf_type == Sphere {
            let size = self.parameters[0].get_float();
            d = length(p - self.position) - size;
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

    /*
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

                    let q = abs(vec2f(x as f32 - rect.x as f32, y as f32 - rect.y as f32) - vec2f(half, half)) - vec2f(size, size);
                    let d = length(max(q,Vec2f::new(0.0, 0.0))) + min(max(q.x,q.y),0.0);

                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        } else
        if self.sdf_type == Sphere {
            let size = half - 5.0;
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    let i = x * 4 + y * stride * 4;
                    let d = length(vec2f(x as f32 - rect.x as f32, y as f32 - rect.y as f32) - vec2f(half, half)) - size;
                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        }

    }*/
}

