use crate::prelude::*;

use strum_macros::{EnumIter, Display};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, EnumIter, Display)]
pub enum PatternType {
    Solid,
    Noise
}

use PatternType::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Pattern {
    pattern_type                : PatternType,

    pub position                : Vec2f,
    pub size                    : Vec2f,
}

impl Pattern {
    pub fn new(pattern_type: PatternType) -> Self {
        Self {
            pattern_type,

            position            : Vec2f::new(0.5, 0.5),
            size                : Vec2f::new(0.4, 0.4),
        }
    }

    pub fn color(&self, _p: Vec2f) -> u8 {
        // let mut d = std::f32::MAX;
        // if self.sdf_type == Box {
        //     let q = abs(p - self.position) - self.size / 2.0;
        //     d = length(max(q,Vec2f::new(0.0, 0.0))) + min(max(q.x,q.y),0.0);
        // } else
        // if self.sdf_type == Circle {
        //     d = length(p - self.position) - self.size.x / 2.0;
        // }
        // d
        50
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

        if self.pattern_type == Solid {
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
        if self.pattern_type == Noise {
            let size = half - 5.0;
            for y in rect.y..rect.y + rect.height {
                for x in rect.x..rect.x + rect.width {
                    let i = x * 4 + y * stride * 4;
                    let d = length(vec2f(x as f32 - rect.x as f32, y as f32 - rect.y as f32) - vec2f(half, half)) - size;
                    pixels[i..i + 4].copy_from_slice(&shade(d));
                }
            }
        }

    }
}
