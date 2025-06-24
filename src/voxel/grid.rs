use crate::prelude::*;
use theframework::prelude::FxHashMap;
use vek::{Aabb, Vec3};

type Coord = (i32, i32, i32);

/// The voxel overlay used by tools.
#[derive(Default)]
struct Overlay {
    pub added: FxHashMap<Coord, u8>, // Added voxels
    pub removed: FxHashSet<Coord>,   // Removed voxels
}

impl Overlay {
    fn empty() -> Self {
        Self {
            added: FxHashMap::default(),
            removed: FxHashSet::default(),
        }
    }

    pub fn clear(&mut self) {
        self.added.clear();
        self.removed.clear();
    }
}

/// One committed action – needed for undo / redo
#[derive(Default)]
struct Diff {
    changes: FxHashMap<Coord, Change>,
}

/// The change diff for a specific Coord.
#[derive(Clone, Copy)]
struct Change {
    prev: Option<u8>, // state *before* the action
    new: Option<u8>,  // state *after*  the action
}

/// Sparse VoxelGrid
pub struct VoxelGrid {
    voxels: FxHashMap<Coord, u8>,
    preview: Overlay,
    undo_stack: Vec<Diff>, // past actions
    redo_stack: Vec<Diff>, // actions we undid
    bounds: [F; 3],        // world-space size
    size: [usize; 3],      // voxel resolution   (nx,ny,nz)
    pub density: usize,    // voxels per world-unit
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new([2.0, 2.0, 2.0], 128)
    }
}

impl VoxelGrid {
    pub fn new(bounds: [F; 3], density: usize) -> Self {
        let sz = [
            (bounds[0] * density as F).ceil() as usize,
            (bounds[1] * density as F).ceil() as usize,
            (bounds[2] * density as F).ceil() as usize,
        ];
        Self {
            voxels: FxHashMap::default(),
            preview: Overlay::empty(),
            undo_stack: Default::default(),
            redo_stack: Default::default(),
            bounds,
            size: sz,
            density,
        }
    }

    /// Bounding box
    #[inline]
    pub fn bbox(&self) -> Aabb<F> {
        let h = Vec3::from(self.bounds) * 0.5;
        Aabb { min: -h, max: h } // centred in **all** axes
    }

    /// world **corner** of voxel `(ix,iy,iz)`
    #[inline]
    fn index_to_corner(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        (Vec3::new(ix as F, iy as F, iz as F) * self.voxel_size()) - Vec3::from(self.bounds) * 0.5
    }

    /// world **centre** of voxel `(ix,iy,iz)`
    #[inline]
    fn index_to_centre(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        self.index_to_corner(ix, iy, iz) + self.voxel_size() * 0.5
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
    pub fn world_to_index(&self, p: Vec3<F>) -> Option<(i32, i32, i32)> {
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

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, m: u8) {
        self.voxels.insert((x, y, z), m);
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        let key = (x, y, z);

        // Preview added ?
        if let Some(&m) = self.preview.added.get(&key) {
            return Some(m);
        }
        // Preview removed ?
        if self.preview.removed.contains(&key) {
            return None;
        }
        // Committed
        self.voxels.get(&key).copied()
    }

    /// Stamp *new* voxels into the overlay
    pub fn preview_add(&mut self, x: i32, y: i32, z: i32, mat: u8) {
        self.preview.added.insert((x, y, z), mat);
        self.preview.removed.remove(&(x, y, z)); // in case it was flagged “remove”
    }

    /// Mark an existing voxel to disappear in the preview
    pub fn preview_remove(&mut self, x: i32, y: i32, z: i32) {
        if self.voxels.contains_key(&(x, y, z)) {
            self.preview.removed.insert((x, y, z));
            self.preview.added.remove(&(x, y, z)); // prevents “add & remove” clash
        }
    }

    /// Clear the preview
    pub fn clear_preview(&mut self) {
        self.preview.clear();
    }

    /// Apply the current preview to the base layer
    pub fn commit_preview(&mut self) {
        if self.preview.added.is_empty() && self.preview.removed.is_empty() {
            return;
        }

        let mut diff = Diff::default();

        // Deletions
        for key in &self.preview.removed {
            let prev = self.voxels.remove(key);
            diff.changes.insert(*key, Change { prev, new: None });
        }

        // Additions / replacements
        for (&key, &mat) in &self.preview.added {
            let prev = self.voxels.insert(key, mat); // insert/overwrite
            diff.changes.insert(
                key,
                Change {
                    prev,
                    new: Some(mat),
                },
            );
        }

        // push to undo stack & clear redo (branch)
        self.undo_stack.push(diff);
        self.redo_stack.clear();
        self.clear_preview();
    }

    /// Undo
    pub fn undo(&mut self) -> bool {
        if let Some(diff) = self.undo_stack.pop() {
            // roll back every change
            for (k, ch) in &diff.changes {
                match ch.prev {
                    Some(mat) => {
                        self.voxels.insert(*k, mat);
                    }
                    None => {
                        self.voxels.remove(k);
                    }
                }
            }
            self.redo_stack.push(diff);
            true
        } else {
            false
        }
    }

    /// Redo
    pub fn redo(&mut self) -> bool {
        if let Some(diff) = self.redo_stack.pop() {
            // re-apply every change
            for (k, ch) in &diff.changes {
                match ch.new {
                    Some(mat) => {
                        self.voxels.insert(*k, mat);
                    }
                    None => {
                        self.voxels.remove(k);
                    }
                }
            }
            self.undo_stack.push(diff);
            true
        } else {
            false
        }
    }

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

    /// Voxel accurate DDA
    pub fn dda(&self, ray: &Ray) -> HitRecord {
        let (mut t_min, t_max) = match ray.intersect_aabb(&self.bbox()) {
            Some(bounds) => bounds,
            None => return HitRecord::default(),
        };

        if t_min < 0.0 {
            t_min = 0.0;
        }

        // Advance to first voxel
        let eps = 1e-4;
        let p_w = ray.at(&(t_min + eps));
        let (ix, iy, iz) = match self.world_to_index(p_w) {
            Some(idx) => idx,
            None => return HitRecord::default(),
        };

        // Pre-computation
        let vs = self.voxel_size();
        let step_i = ray.dir.map(|d| if d < 0.0 { -1 } else { 1 });
        let inv_dir = ray.dir.map(|d| 1.0 / d.abs().max(1e-30));
        let t_delta = vs * inv_dir; // Δt to cross one voxel

        // Distance to first voxel boundary
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

        // Integer bounds
        let max_i = Vec3::new(
            self.size[0] as i32 - 1,
            self.size[1] as i32 - 1,
            self.size[2] as i32 - 1,
        );

        // Loop
        let mut pos = Vec3::new(ix, iy, iz);
        let mut t = t_min + eps;
        let t_end = t_max.min(1_000.0);

        while t <= t_end {
            // out-of-bounds test
            if pos.x < 0
                || pos.y < 0
                || pos.z < 0
                || pos.x > max_i.x
                || pos.y > max_i.y
                || pos.z > max_i.z
            {
                return HitRecord {
                    hit: HitType::BBox((t_min, t_max)),
                    ..Default::default()
                };
            }

            // Hit
            if let Some(m) = self.get(pos.x, pos.y, pos.z) {
                let hit_world = ray.at(&t);
                let centre = self.index_to_centre(pos.x, pos.y, pos.z);
                let delta = hit_world - centre;
                let (n, face) = if delta.x.abs() > delta.y.abs() && delta.x.abs() > delta.z.abs() {
                    (
                        Vec3::new(delta.x.signum(), 0.0, 0.0),
                        if delta.x > 0.0 { Face::PX } else { Face::NX },
                    )
                } else if delta.y.abs() > delta.z.abs() {
                    (
                        Vec3::new(0.0, delta.y.signum(), 0.0),
                        if delta.y > 0.0 { Face::PY } else { Face::NY },
                    )
                } else {
                    (
                        Vec3::new(0.0, 0.0, delta.z.signum()),
                        if delta.z > 0.0 { Face::PZ } else { Face::NZ },
                    )
                };

                let hs = HitRecord {
                    hit_point_local: pos,
                    hit_point_world: hit_world,
                    normal: n,
                    face,
                    hit: HitType::Voxel(m),
                };
                return hs;
            }

            // Advance to next voxel
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

        HitRecord {
            hit: HitType::BBox((t_min, t_max)),
            ..Default::default()
        }
    }
}
