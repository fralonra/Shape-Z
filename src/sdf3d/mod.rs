use crate::prelude::*;

//pub mod wall;
//pub mod voxels;

pub mod prelude {
    pub use crate::sdf3d::SDF3D;
}

#[allow(unused)]
pub trait SDF3D : Sync + Send {
    fn new() -> Self where Self: Sized;

    fn id(&self) -> Uuid;

    fn name(&self) -> String;

    fn get_properties(&self) -> Properties;
    fn set_properties(&mut self, properties: Properties);

    fn get_properties_ref(&self) -> &Properties;

    fn update(&mut self);

    fn get_color(&self, hit: &HitRecord) -> u8;

    /*
    /// Default uv mapping
    fn uv_cube(&self, hp: &GF3, normal: &GF3) -> (GF2, u8) {
        if normal.y > 0.5 {
            // Top
            let uv = GF2::new((hp.x + 0.5), (hp.z + 0.5));
            let dim = self.get_properties_ref().color_top.dimension;
            let dim_f = dim as F;

            let x = ((uv.x * dim_f) as usize).min(dim - 1);
            let y = ((uv.y * dim_f) as usize).min(dim - 1);

            let color = self.get_properties_ref().color_top.pixels[x + y * self.get_properties_ref().color_top.dimension];
            (uv, color)
        } else
        if normal.z > 0.5 {
            // Front
            //GF2::new(1.0 - (hp.x + 0.5), 1.0 - (hp.y + 0.5))
            let uv = GF2::new((hp.x + 0.5), 1.0 - (hp.y + 0.5));
            let dim = self.get_properties_ref().color_front.dimension;
            let dim_f = dim as F;

            let x = ((uv.x * dim_f) as usize).min(dim - 1);
            let y = ((uv.y * dim_f) as usize).min(dim - 1);

            let color = self.get_properties_ref().color_front.pixels[x + y * self.get_properties_ref().color_front.dimension];
            (uv, color)
        } else {
            // Side
            //GF2::new(1.0 - (hp.z + 0.5), 1.0 - (hp.y + 0.5))
            let uv = GF2::new(1.0 - (hp.z + 0.5), 1.0 - (hp.y + 0.5));
            let dim = self.get_properties_ref().color_side.dimension;
            let dim_f = dim as F;

            let x = ((uv.x * dim_f) as usize).min(dim - 1);
            let y = ((uv.y * dim_f) as usize).min(dim - 1);

            let color = self.get_properties_ref().color_side.pixels[x + y * self.get_properties_ref().color_side.dimension];
            (uv, color)
        }
    }*/

    fn create(&self) -> Box<dyn SDF3D>;

    #[inline(always)]
    fn distance(&self, x: Vec3f, inst: Vec3f) -> f32;
}