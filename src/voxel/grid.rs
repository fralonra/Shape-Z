use crate::prelude::*;
use theframework::prelude::FxHashMap;
use vek::{Aabb, Vec3};

pub struct VoxelGrid {
    voxels: FxHashMap<(i32, i32, i32), u8>,
    bounds: [F; 3], // world-space size   (units)
    size: [usize; 3], // voxel resolution   (nx,ny,nz)
                    // density: usize,   // voxels per world-unit
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new([2.0, 2.0, 2.0], 128)
    }
}

/* ──────────────────────────────────────────────────────────────────────── */

impl VoxelGrid {
    pub fn new(bounds: [F; 3], density: usize) -> Self {
        let sz = [
            (bounds[0] * density as F).ceil() as usize,
            (bounds[1] * density as F).ceil() as usize,
            (bounds[2] * density as F).ceil() as usize,
        ];
        Self {
            voxels: FxHashMap::default(),
            bounds,
            size: sz,
            // density,
        }
    }

    /* ── constants & helpers ─────────────────────────────────────────── */

    #[inline]
    fn vs(&self) -> Vec3<F> {
        Vec3::new(
            self.bounds[0] / self.size[0] as F,
            self.bounds[1] / self.size[1] as F,
            self.bounds[2] / self.size[2] as F,
        )
    }

    #[inline]
    fn bbox(&self) -> Aabb<F> {
        let h = Vec3::from(self.bounds) * 0.5;
        Aabb { min: -h, max: h } // centred in **all** axes
    }

    /* ── world  ↔  index ─────────────────────────────────────────────── */

    /// world **corner** of voxel `(ix,iy,iz)`
    #[inline]
    fn index_to_corner(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        (Vec3::new(ix as F, iy as F, iz as F) * self.vs()) - Vec3::from(self.bounds) * 0.5
    }

    /// world **centre** of voxel `(ix,iy,iz)`
    #[inline]
    fn index_to_centre(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        self.index_to_corner(ix, iy, iz) + self.vs() * 0.5
    }

    /// world → voxel index (**corner-based**, floor)
    /// length of a voxel along each axis (world units)
    #[inline]
    fn voxel_size(&self) -> Vec3<F> {
        Vec3::new(
            self.bounds[0] / self.size[0] as F,
            self.bounds[1] / self.size[1] as F,
            self.bounds[2] / self.size[2] as F,
        )
    }

    /// world position → voxel corner index  (returns None if outside)
    #[inline]
    fn world_to_index(&self, p: Vec3<F>) -> Option<(i32, i32, i32)> {
        // shift the grid so its centre is at the origin → everything becomes ≥0
        let shifted = p + Vec3::new(self.bounds[0], self.bounds[1], self.bounds[2]) * 0.5;

        // divide by voxel size & floor to get integer coordinates
        let vs = self.voxel_size();
        let v = (shifted / vs).map(|c| c.floor() as i32);

        // manual bounds check (avoids unstable cmp… intrinsics)
        if v.x >= 0
            && v.x < self.size[0] as i32
            && v.y >= 0
            && v.y < self.size[1] as i32
            && v.z >= 0
            && v.z < self.size[2] as i32
        {
            Some((v.x, v.y, v.z))
        } else {
            None
        }
    }

    /* ── sparse access ──────────────────────────────────────────────── */

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, m: u8) {
        self.voxels.insert((x, y, z), m);
    }
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        self.voxels.get(&(x, y, z)).copied()
    }

    /* ── primitives ─────────────────────────────────────────────────── */

    /// Fills a solid sphere (centre & radius in *world* units).
    pub fn add_sphere(&mut self, centre: Vec3<F>, r: F, mat: u8) {
        let r2 = r * r;
        let min_ix = self
            .world_to_index(centre - Vec3::broadcast(r))
            .unwrap_or((0, 0, 0));
        let max_ix = self
            .world_to_index(centre + Vec3::broadcast(r) - 1e-6)
            .unwrap_or((
                self.size[0] as i32 - 1,
                self.size[1] as i32 - 1,
                self.size[2] as i32 - 1,
            ));

        for z in min_ix.2..=max_ix.2 {
            for y in min_ix.1..=max_ix.1 {
                for x in min_ix.0..=max_ix.0 {
                    if (self.index_to_centre(x, y, z) - centre).magnitude_squared() <= r2 {
                        self.set(x, y, z, mat);
                    }
                }
            }
        }
    }

    /* ── voxel-accurate DDA ──────────────────────────────────────────── */

    pub fn dda(&self, ray: &Ray) -> Option<u8> {
        /* 1. clip against grid AABB */
        let (mut t_min, t_max) = ray.intersect_aabb(&self.bbox())?;
        if t_min < 0.0 {
            t_min = 0.0;
        }

        /* 2. first voxel the ray is inside */
        let eps = 1e-4;
        let p_w = ray.at(&(t_min + eps));
        let (ix, iy, iz) = self.world_to_index(p_w)?;

        /* 3. DDA pre-computation */
        let vs = self.vs();
        let step_i = ray.dir.map(|d| if d < 0.0 { -1 } else { 1 });
        let inv_dir = ray.dir.map(|d| 1.0 / d.abs().max(1e-30));
        let t_delta = vs * inv_dir; // Δt to cross one voxel

        // distance to first voxel boundary
        let centre = self.index_to_centre(ix, iy, iz);
        let mut t_max_v = Vec3::zero();
        for a in 0..3 {
            let half = vs[a] * 0.5;
            let bound = if step_i[a] > 0 {
                centre[a] + half
            } else {
                centre[a] - half
            };
            t_max_v[a] = (bound - p_w[a]).abs() * inv_dir[a];
        }

        /* 4. integer bounds */
        let max_i = Vec3::new(
            self.size[0] as i32 - 1,
            self.size[1] as i32 - 1,
            self.size[2] as i32 - 1,
        );

        /* 5. loop */
        let mut pos = Vec3::new(ix, iy, iz);
        let mut t = t_min + eps;
        let t_end = t_max.min(1_000.0);

        while t <= t_end {
            /* out-of-bounds test (all axes identical) */
            if pos.x < 0
                || pos.y < 0
                || pos.z < 0
                || pos.x > max_i.x
                || pos.y > max_i.y
                || pos.z > max_i.z
            {
                return None;
            }

            /* hit */
            if let Some(m) = self.get(pos.x, pos.y, pos.z) {
                return Some(m);
            }

            /* advance to next voxel */
            if t_max_v.x < t_max_v.y {
                if t_max_v.x < t_max_v.z {
                    pos.x += step_i.x;
                    t = t_max_v.x;
                    t_max_v.x += t_delta.x;
                } else {
                    pos.z += step_i.z;
                    t = t_max_v.z;
                    t_max_v.z += t_delta.z;
                }
            } else if t_max_v.y < t_max_v.z {
                pos.y += step_i.y;
                t = t_max_v.y;
                t_max_v.y += t_delta.y;
            } else {
                pos.z += step_i.z;
                t = t_max_v.z;
                t_max_v.z += t_delta.z;
            }
        }
        None
    }
}
