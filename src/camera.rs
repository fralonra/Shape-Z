use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Ray {
    pub o           : Vec3f,
    pub d           : Vec3f,
}

impl Ray {

    pub fn new(o : Vec3f, d : Vec3f) -> Self {
        Self {
            o,
            d,
        }
    }

    /// Returns the position on the ray at the given distance
    pub fn at(&self, d: f32) -> Vec3f {
        self.o + self.d * d
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Camera {
    pub origin      : Vec3f,
    pub center      : Vec3f,
    pub fov         : f32,
}

impl Camera {

    pub fn new(origin: Vec3f, center: Vec3f, fov: f32) -> Self {
        Self {
            origin,
            center,
            fov
        }
    }

    /// Create a pinhole ray
    pub fn create_ray(&self, uv: Vec2f, screen: Vec2f) -> Ray {
        let ratio = screen.x / screen.y;
        let pixel_size = vec2f(1.0 / screen.x, 1.0 / screen.y);

        let half_width = (self.fov.to_radians() * 0.5).tan();
        let half_height = half_width / ratio;

        let up_vector = vec3f(0.0, 1.0, 0.0);

        let w = normalize(self.origin - self.center);
        let u = cross(up_vector, w);
        let v = cross(w, u);

        let lower_left = self.origin - u * half_width - v * half_height - w;
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;
        let mut dir = lower_left - self.origin;
        let rand = vec2f(0.5, 0.5);

        dir += horizontal * (pixel_size.x * rand.x + uv.x);
        dir += vertical * (pixel_size.y * rand.y + uv.y);

        Ray::new(self.origin, normalize(dir))
    }
}
