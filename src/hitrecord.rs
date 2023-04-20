use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct HitRecord {
    pub hitpoint        : Vec3f,
    pub key             : Vec3i,
    pub distance        : f32,
    pub normal          : Vec3f,
    pub uv              : Vec3f,

}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            hitpoint    : Vec3f::zero(),
            key         : Vec3i::zero(),
            distance    : 0.0,
            normal      : Vec3f::zero(),
            uv          : Vec3f::zero(),
        }
    }
}