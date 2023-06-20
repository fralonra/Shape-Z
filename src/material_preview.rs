use crate::prelude::*;
use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};
use rand::{thread_rng, Rng, rngs::ThreadRng};

#[derive(PartialEq, Debug, Clone)]
pub struct MaterialPreview {

    pub camera              : Camera,

    pub color               : u8,
    pub material            : u8,
}

impl MaterialPreview {
    pub fn new() -> Self {

        let camera = Camera::new(Vec3f::new(0.0, 0.0, 2.0), Vec3f::new(0.0, 0.0, 0.0), 45.0);

        Self {
            camera          : camera,

            color           : 0,
            material        : 100,
        }
    }

    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    pub fn render(&mut self, buffer: &mut ColorBuffer, context: &Context) {

        let width = buffer.width;
        let height = buffer.height as f32;

        let screen = vec2f(buffer.width as f32, buffer.height as f32);

        //let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;
        //let _start = self.get_time();

        let sp_center = Vec3f::zero();
        let sp_radius = 0.5;
        let sp_color = context.palette.at_vec_to_linear(self.color);

        let material = &context.materials[self.material as usize];

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {

                    #[inline(always)]
                    pub fn mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let mut rng: ThreadRng = thread_rng();

                    let mut sum = [0.0, 0.0, 0.0, 1.0];

                    let max_depth = 1;

                    let pi = std::f32::consts::PI;

                    for i in 0..100 {

                        let cam_off = vec2f(rng.gen(), rng.gen());
                        // let mut ray = self.camera.create_orbit_ray(uv, screen, cam_off);
                        let mut ray = self.camera.create_ray(uv, screen, cam_off);

                        let mut color;
                        let mut hit_something = false;

                        let mut acc = Vec3f::zero();
                        let mut mask = Vec3f::one();

                        for _depth in 0..max_depth {

                            if let Some(d) = self.sphere(&ray, sp_center, sp_radius) {

                                hit_something = true;

                                let hp = ray.at(d);
                                let n = normalize(hp - sp_center);
                                let nl = n * signum(-dot(n, ray.d));

                                let roughness = material.roughness;
                                let alpha = roughness * roughness;
                                let metallic = material.metallic;
                                let reflectance = material.reflectance;
                                let diffuse = sp_color;
                                let emission = Vec3f::zero();//material.emission * diffuse;

                                let mut brdf = vec3f(0.0, 0.0, 0.0);

                                let light_pos = vec3f(0.3, 0.1, 2.0);
                                let light_radius = 0.2;
                                let light_emission = vec3f(200.0, 200.0, 200.0);

                                let x = hp - 0.001 * n;

                                if reflectance == 1.0 || rng.gen::<f32>() < reflectance {

                                    #[inline(always)]
                                    pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
                                        (1.0 - v) * a + b * v
                                    }

                                    let l0 = light_pos - x;
                                    let cos_a_max = sqrt(1. - clamp(light_radius * light_radius / dot(l0, l0), 0.0, 1.0));
                                    let cosa = mix(&cos_a_max, &1.0, rng.gen());
                                    let l = jitter(l0, 2.0 * pi * rng.gen::<f32>(), sqrt(1.0 - cosa*cosa), cosa);

                                    // if let Some(_hit_refl) = self.sphere(&Ray::new(x, l), sp_center, sp_radius) {
                                    // } else {
                                        if let Some(_hit_refl) = self.sphere(&Ray::new(x, l), light_pos, light_radius) {
                                            let omega = 2.0 * pi * (1.0 - cos_a_max);
                                            brdf += (light_emission * clamp(ggx(nl, ray.d, l, roughness, metallic),0.0,1.0) * omega) / pi;
                                        }
                                    // }

                                    let xsi_1 = rng.gen::<f32>();
                                    let xsi_2 = rng.gen::<f32>();
                                    let phi = atan((alpha * sqrt(xsi_1)) / sqrt(1.0 - xsi_1));
                                    let theta = 2.0 * pi * xsi_2;
                                    let direction = angle_to_dir(nl, theta, phi);
                                    ray = Ray::new(x, direction);
                                    acc += mask * emission + mask * diffuse * brdf;
                                    mask *= diffuse;
                                } else {

                                    #[inline(always)]
                                    pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
                                        (1.0 - v) * a + b * v
                                    }

                                    let r2 = rng.gen();
                                    let d = jitter(nl, 2.0 * pi * rng.gen::<f32>(), sqrt(r2), sqrt(1.0 - r2));
                                    let mut e = Vec3f::zero();

                                    let l0 = light_pos - x;

                                    let cos_a_max = sqrt(1.0 - clamp(light_radius * light_radius / dot(l0, l0), 0., 1.));
                                    let cosa = mix(&cos_a_max, &1.0, rng.gen());
                                    let l = jitter(l0, 2.0 * pi * rng.gen::<f32>(), sqrt(1.0 - cosa * cosa), cosa);

                                    if let Some(_hit_refl) = self.sphere(&Ray::new(x, l), light_pos, light_radius) {
                                        let omega = 2.0 * pi * (1.0 - cos_a_max);
                                        e += (light_emission * clamp(dot(l, n),0.0,1.0) * omega) / pi;
                                    }

                                    acc += mask * emission + mask * diffuse * e;
                                    mask *= diffuse;
                                    ray = Ray::new(x, d);
                                }
                            } else {
                                //acc += mask * vec3f(0.5, 0.5, 0.5);
                                break;
                            }
                        }

                        color = [acc.x, acc.y, acc.z, 1.0];

                        if hit_something {
                            // Clip color to the palette
                            let index = context.palette.closest(color[0].powf(0.4545), color[1].powf(0.4545), color[2].powf(0.4545));
                            color = context.palette.at_f(index);
                        } else {
                            color = [0.15, 0.15, 0.15, 1.0];
                        }

                        // Accumulate
                        sum = mix_color(&sum, &color, 1.0 / (i + 1) as f32);
                    }
                    pixel.copy_from_slice(&sum);
                }
        });

        //let _stop = self.get_time();
        //println!("renter time {:?}, iter: {}", _stop - _start, iteration);
    }

    // Based on https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
    fn sphere(&self, ray: &Ray, center: Vec3f, radius: f32) -> Option<f32> {
        let l = center - ray.o;
        let tca = dot(l, ray.d);
        let d2 = dot(l,l) - tca * tca;
        let radius2 = radius * radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        Some(t0)
   }

    // Ray plane intersection
    fn _plane(&self, ray: &Ray) -> Option<f32> {
        let normal = Vec3f::new(0.0, 1.0, 0.0);
        let denom = dot(normal, ray.d);

        if denom.abs() > 0.0001 {
            let t = dot(Vec3f::new(0.0, -1.0, 0.0) - ray.o, normal) / denom;
            if t >= 0.0 {
                return Some(t);
            }
        }
        None
    }
}