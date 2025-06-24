use crate::prelude::*;
use vek::{Vec2, Vec3, Vec4};

use rand::Rng;

pub struct EditShader {
    pub background_color: Vec3<F>,
}

impl Renderer for EditShader {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            background_color: Vec3::broadcast(0.2),
        }
    }

    fn name(&self) -> &str {
        "EditShader"
    }

    /// Get the background color.
    fn background_color(&mut self) -> Vec3<F> {
        self.background_color
    }

    /// Set the background color.
    fn set_background_color(&mut self, color: Vec3<F>) {
        self.background_color = color;
    }

    /// Render the pixel at the given screen position.
    fn render(
        &self,
        uv: Vec2<F>,
        resolution: Vec2<F>,
        grid: &VoxelGrid,
        palette: &Palette,
        camera: &Box<dyn Camera>,
    ) -> Vec4<F> {
        let mut rng = rand::rng();

        // let mut acc = Vec3::<F>::zero();
        // let mut mask = Vec3::<F>::one();

        let ray = camera.create_ray(uv, resolution, Vec2::new(rng.random(), rng.random()));

        // -------------- voxel DDA -------------------------------------------
        let hit = grid.dda(&ray);
        match hit.hit {
            HitType::Outside => {
                // background
                let c = self.background_color;
                Vec4::new(c.x, c.y, c.z, 1.0)
            }
            HitType::BBox(_) => {
                // background
                let c = self.background_color;
                Vec4::new(c.x, c.y, c.z, 1.0)
            }
            HitType::Voxel(m) => {
                // -------------- palette lookup --------------------------------------
                let mat = palette.get(m);
                let mut color = mat.base_color; // linear 0-1

                // -------------- basic diffuse lighting ------------------------------
                let light_dir = Vec3::new(-0.5, 1.0, -0.5).normalized();
                let n = hit.normal.normalized();
                let diff = n.dot(light_dir).max(0.0);
                let mut shade = 0.20 /*ambient*/ + 0.80 * diff;

                // -------------- tiny edge/specular accent ---------------------------
                let view_dir = -ray.dir; // pointing to camera
                let spec_boost = view_dir.dot(n).max(0.0).powf(20.0);
                shade += 0.15 * spec_boost; // up to +15 %

                // -------------- 6-tap ambient-occlusion -----------------------------
                let p = hit.hit_point_local; // i32 voxel coords
                let neigh = [
                    Vec3::new(1, 0, 0),
                    Vec3::new(-1, 0, 0),
                    Vec3::new(0, 1, 0),
                    Vec3::new(0, -1, 0),
                    Vec3::new(0, 0, 1),
                    Vec3::new(0, 0, -1),
                ];
                let empty = neigh
                    .iter()
                    .filter(|&&d| grid.get(p.x + d.x, p.y + d.y, p.z + d.z).is_none())
                    .count() as F; // 0‥6
                let ao = 1.0 + empty * 0.09; // up to +54 %
                shade *= ao;

                // -------------- final colour ----------------------------------------
                color *= shade.clamp(0.0, 1.5); // keep sane
                Vec4::new(color.x, color.y, color.z, 1.0)
            }
        }
    }
}
