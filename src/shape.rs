use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Shape {

    pub position                : Vec3f,
    pub sdf                     : Vec<Option<SDF>>,
}

impl Shape {
    pub fn new() -> Self {

        let mut sdf = vec![None; 5];

        sdf[0] = Some(SDF::new(SDFType::Box));

        Self {
            position            : Vec3f::zero(),

            sdf,
        }
    }

    /// Sets an SDF
    pub fn set_sdf(&mut self, index: usize, primitive: Option<SDF>) {
        self.sdf[index] = primitive;
    }

    pub fn draw(&mut self, pixels: &mut [u8], rect: Rect, stride: usize, context: &mut Context, ctx: &TheContext) {

        let r = rect.to_usize();

        ctx.draw.rect(pixels, &r, stride, &context.color_black);
    }
}
