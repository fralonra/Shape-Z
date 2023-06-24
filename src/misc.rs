use crate::prelude::*;

/*
vec2 hash2(vec3 p3) {
	p3 = fract(p3 * vec3(5.3983, 5.4427, 6.9371));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.xx + p3.yz) * p3.zy);
}*/

pub fn hash3_2(mut p3: Vec3f) -> Vec2f {
    p3 = frac(p3 * Vec3f::new(5.3983, 5.4427, 6.9371 ));
    p3 += dot(p3, p3.yzx() + 19.19);
    frac((p3.xx() + p3.yz()) * p3.zy())
}

/// AABB
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AABB {
    pub min             : Vec3f,
    pub max             : Vec3f,
}

use std::ops::Index;

impl Index<usize> for AABB {
    type Output = Vec3f;

    fn index(&self, index: usize) -> &Vec3f {
        if index == 0 {
            &self.min
        } else {
            &self.max
        }
    }
}

/// Ray
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Ray {
    pub o                   : Vec3f,
    pub d                   : Vec3f,

    pub inv_direction       : Vec3f,

    pub sign_x              : usize,
    pub sign_y              : usize,
    pub sign_z              : usize,
}

impl Ray {

    pub fn new(o : Vec3f, d : Vec3f) -> Self {
        Self {
            o,
            d,

            inv_direction   : Vec3f::new(1.0 / d.x, 1.0 / d.y, 1.0 / d.z),
            sign_x          : (d.x < 0.0) as usize,
            sign_y          : (d.y < 0.0) as usize,
            sign_z          : (d.z < 0.0) as usize
        }
    }

    /// Returns the position on the ray at the given distance
    pub fn at(&self, d: f32) -> Vec3f {
        self.o + self.d * d
    }
}

use strum::IntoEnumIterator;

/// Create the previews for the 2D primitives
pub fn create_shape_previews() -> Vec<u8> {
     let amount = SDFType::iter().len();

    let size = 40;
    let mut rect = Rect::new(0, 0, size, size * amount);
    let mut buff = rect.alloc();

    for sdf_type in SDFType::iter() {
        let shape = SDF::new(sdf_type);
        shape.create_preview(&mut buff, rect, size);
        rect.y += size;
    }

    buff
}
