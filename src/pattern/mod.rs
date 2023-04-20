use crate::prelude::*;

pub mod brick;
pub mod value;
pub mod voronoi;

pub mod prelude {
    pub use crate::pattern::Pattern;
    pub use crate::pattern::brick::Brick;
    pub use crate::pattern::value::Value;
    pub use crate::pattern::voronoi::Voronoi;
}

#[allow(unused)]
pub trait Pattern : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn id(&self) -> Uuid;

    fn name(&self) -> String;

    fn get_color(&self, uv: Vec2f) -> u8;

    /// 2D hash, taken from https://www.shadertoy.com/view/4djSRW
    #[inline(always)]
    fn hash21(&self, p: Vec2f) -> f32 {
        let mut p3 = frac(Vec3f::new(p.x * 0.1031, p.y * 0.1031, p.x * 0.1031));

        let dot = dot(p3, Vec3f::new(p3.y + 33.333, p3.z + 33.333, p3.x + 33.333));

        p3.x += dot; p3.y += dot; p3.z += dot;
        ((p3.x + p3.y) * p3.z).fract()
    }

    /// Mix for FP
    #[inline(always)]
    fn mix(&self, a: &f32, b: &f32, v: &f32) -> f32 {
        (1.0 - v) * a + b * v
    }

    fn create(&self) -> Box::<dyn Pattern>;
}