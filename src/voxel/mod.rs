pub mod camera;
pub mod grid;
pub mod palette;
pub mod ray;
pub mod renderbuffer;
pub mod renderer;

use crate::F;
use vek::Vec3;

/// Face
#[derive(Debug, Clone, Copy)]
pub enum Face {
    PX,
    NX,
    PY,
    NY,
    PZ,
    NZ,
}

/// HitType
#[derive(Debug, Clone, PartialEq)]
pub enum HitType {
    Voxel(u8),        // Material ID
    BBox((f32, f32)), // BBox hit (t_min, t_far)
    Outside,          // Ray didn't enter grid
}

/// HitRecord
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub hit_point_local: Vec3<i32>,
    pub hit_point_world: Vec3<F>,
    pub normal: Vec3<F>,
    pub face: Face,
    pub hit: HitType,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            hit_point_local: Vec3::zero(),
            hit_point_world: Vec3::zero(),
            normal: Vec3::zero(),
            face: Face::NX,
            hit: HitType::Outside,
        }
    }
}
