use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Shape {

    pub position                : Vec3f,
    pub sdf                     : Vec<Option<SDF>>,

    pub size                    : i32,
}

impl Shape {
    pub fn new() -> Self {

        let mut sdf = vec![None; 5];

        sdf[0] = Some(SDF::new(SDFType::Circle));

        Self {
            position            : Vec3f::zero(),

            sdf,

            size                : 10,
        }
    }

    /// Sets an SDF
    pub fn set_sdf(&mut self, index: usize, primitive: Option<SDF>) {
        self.sdf[index] = primitive;
    }

    pub fn draw(&mut self, pixels: &mut [u8], rect: Rect, stride: usize, context: &mut Context, _ctx: &TheContext) {

        let width_f = rect.width as f32;
        let height_f = rect.height as f32;

        let a  = context.palette.at_f(0);
        let b = context.palette.at_f(100);

        #[inline(always)]
        pub fn mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
            [   (1.0 - v) * a[0] + b[0] * v,
                (1.0 - v) * a[1] + b[1] * v,
                (1.0 - v) * a[2] + b[2] * v,
                (1.0 - v) * a[3] + b[3] * v ]
        }

        for y in 0..rect.height {
            for x in 0..rect.width {
                let mut d: f32 = std::f32::MAX;
                let xx = x as f32;
                let yy = y as f32;

                let p = vec2f(xx / width_f, yy / height_f);

                let mut hit_index : Option<usize>;// = None;

                for (index, sdf) in self.sdf.iter().enumerate() {
                    if let Some(sdf) = sdf {
                        let dd = sdf.distance(p, 1.0);
                        if dd < d {
                            hit_index = Some(index);
                            d = dd;
                        }
                    }
                }

        // let mask: f32 = 1.0 - smoothstep(-2.0, 0.0, *distance);

                let c = mix_color(&a, &b, 1.0 - (smoothstep(-0.01, 0.0, d)));
                let dc = [(c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8, (c[3] * 255.0) as u8];

                let i = (rect.x + x) * 4 + (rect.y + y) * stride * 4;
                pixels[i..i+4].copy_from_slice(&dc);

                /*
                if let Some(hit_index) = hit_index {
                    ctx.draw.rect(pixels, &(rect.x + x, rect.y + y, 1, 1), stride, &context.color_white);
                } else {
                    ctx.draw.rect(pixels, &(rect.x + x, rect.y + y, 1, 1), stride, &context.color_black);
                }*/
            }
        }
    }

    /// Returns the shape index for the given location
    pub fn distance(&self, p: Vec2f) -> (f32, Option<usize>) {
        let mut hit_index : Option<usize> = None;
        let mut d: f32 = std::f32::MAX;
        for (index, sdf) in self.sdf.iter().enumerate() {
            if let Some(sdf) = sdf {
                let dd = sdf.distance(p, 2.0);
                if dd < 0.0 && dd < d {
                    hit_index = Some(index);
                    d = dd;
                }
            }
        }

        (d, hit_index)
    }
}
