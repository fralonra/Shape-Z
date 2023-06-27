use crate::prelude::*;

pub mod text_button;
pub mod settings;
pub mod browser;
pub mod palettebar;
pub mod shape_selector;
pub mod switch_button;
pub mod int_slider;
pub mod value_list;

pub mod tool_extrusion;

#[derive(Clone, Debug)]

pub enum WidgetValue {
    Color(String, u8),
    Material(String, u8)
}

#[allow(unused)]
pub trait Widget : Sync + Send {

    fn new() -> Self where Self: Sized;

    fn set_rect(&mut self, rect: Rect);
    fn set_text(&mut self, text: String) {}
    fn set_text_list(&mut self, text_list: Vec<String>) {}
    fn set_value_list(&mut self, values: Vec<Value>) {}

    fn set_range(&mut self, min: i32, max: i32) {}

    fn get_value(&mut self) -> i32{ 0 }
    fn set_value(&mut self, value: i32) {}

    fn set_cmd(&mut self, cmd: Command) {}

    fn get_state(&self) -> bool { false }
    fn set_has_state(&mut self, state: bool) {}

    fn is_visible(&self) -> bool { return true; }
    fn set_visible(&mut self, visible: bool) { }

    fn update(&mut self, context: &mut Context) {}

    fn draw(&mut self, pixels: &mut [u8], stride: usize, context: &mut Context, world: &World, ctx: &TheContext);

    fn contains(&mut self, x: f32, y: f32) -> bool {
        false
    }

    fn touch_down(&mut self, x: f32, y: f32, context: &mut Context, world: &World) -> bool {
        false
    }

    fn touch_dragged(&mut self, x: f32, y: f32, context: &mut Context) -> bool {
        false
    }

    fn touch_up(&mut self, _x: f32, _y: f32, context: &mut Context) -> bool {
        false
    }

    fn generate_value(&self) -> Value {
        Value::Int("".to_string(), 0, 0, 0)
    }

    fn generate_value_list(&self) -> Vec<Value> {
        vec![]
    }

    // Only for Tools

    fn set_shape(&mut self, shape: Shape) {
    }

    fn sdf_triggered(&mut self, sdf: SDFType) {

    }

    fn hit(&mut self, hit: HitRecord, tiles: &Vec<Vec3i>) {
    }

    // Custom drawing

    /// Draws a rounded rect
    fn draw_rounded_rect_x_limit(&self, frame: &mut [u8], rect: &(usize, usize, usize, usize), stride: usize, color: &[u8; 4], rounding: &(f32, f32, f32, f32), x_limit: usize) {
        let center = ((rect.0 as f32 + rect.2 as f32 / 2.0).round(), (rect.1 as f32 + rect.3 as f32 / 2.0).round());
        for y in rect.1..rect.1+rect.3 {
            for x in rect.0..rect.0+rect.2 {
                let i = x * 4 + y * stride * 4;

                if x > rect.0 + x_limit {
                    break;
                }

                let p = (x as f32 - center.0, y as f32 - center.1);
                let mut r : (f32, f32);

                if p.0 > 0.0 {
                    r = (rounding.0, rounding.1);
                } else {
                    r = (rounding.2, rounding.3);
                }

                if p.1 <= 0.0 {
                    r.0 = r.1;
                }

                let q : (f32, f32) = (p.0.abs() - rect.2 as f32 / 2.0 + r.0, p.1.abs() - rect.3 as f32 / 2.0 + r.0);
                let d = f32::min(f32::max(q.0, q.1), 0.0) + self.length((f32::max(q.0, 0.0), f32::max(q.1, 0.0))) - r.0;

                if d < 0.0 {
                    let t = self.fill_mask(d);

                    let background = &[frame[i], frame[i+1], frame[i+2], 255];
                    let mut mixed_color = self.mix_color(&background, &color, t * (color[3] as f32 / 255.0));
                    mixed_color[3] = (mixed_color[3] as f32 * (color[3] as f32 / 255.0)) as u8;
                    frame[i..i + 4].copy_from_slice(&mixed_color);
                }
            }
        }
    }

    /// The fill mask for an SDF distance
    fn fill_mask(&self, dist : f32) -> f32 {
        (-dist).clamp(0.0, 1.0)
    }

    /// The border mask for an SDF distance
    fn border_mask(&self, dist : f32, width: f32) -> f32 {
       (dist + width).clamp(0.0, 1.0) - dist.clamp(0.0, 1.0)
    }

    /// Smoothstep for f32
    fn _smoothstep(&self, e0: f32, e1: f32, x: f32) -> f32 {
        let t = ((x - e0) / (e1 - e0)). clamp(0.0, 1.0);
        return t * t * (3.0 - 2.0 * t);
    }

    /// Mixes two colors based on v
    fn mix_color(&self, a: &[u8;4], b: &[u8;4], v: f32) -> [u8; 4] {
        [   (((1.0 - v) * (a[0] as f32 / 255.0) + b[0] as f32 / 255.0 * v) * 255.0) as u8,
            (((1.0 - v) * (a[1] as f32 / 255.0) + b[1] as f32 / 255.0 * v) * 255.0) as u8,
            (((1.0 - v) * (a[2] as f32 / 255.0) + b[2] as f32 / 255.0 * v) * 255.0) as u8,
        255]
    }

    // Length of a 2d vector
    fn length(&self, v: (f32, f32)) -> f32 {
        ((v.0).powf(2.0) + (v.1).powf(2.0)).sqrt()
    }
}