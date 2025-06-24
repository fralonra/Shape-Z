use crate::prelude::*;
use std::mem;
use theframework::prelude::FxHashMap;
use vek::{Aabb, Vec3};

pub type Coord = (i32, i32, i32); // voxel key
pub type ChunkCoord = (i32, i32, i32); // chunk key

// --------------------------------------------------------------------------
// Chunk: dense 32³ + small meta data
// --------------------------------------------------------------------------
const CHUNK_LG: usize = 5; // 2⁵ = 32
const CHUNK_SIDE: usize = 1 << CHUNK_LG; // 32
const CHUNK_MASK: i32 = (CHUNK_SIDE - 1) as i32;

#[derive(Clone)]
struct Chunk {
    vox: FxHashMap<Coord, u8>,
}

impl Chunk {
    fn new() -> Self {
        Self {
            vox: FxHashMap::default(),
        }
    }

    #[inline]
    fn get(&self, local: Coord) -> Option<u8> {
        self.vox.get(&local).copied()
    }

    #[inline]
    fn set(&mut self, local: Coord, mat: u8) -> Option<u8> {
        if mat == 255 {
            self.vox.remove(&local)
        } else {
            self.vox.insert(local, mat)
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.vox.is_empty()
    }
}

// --------------------------------------------------------------------------
// Overlay & undo/redo identical to the old version
// --------------------------------------------------------------------------
#[derive(Default)]
struct Overlay {
    added: FxHashMap<Coord, u8>,
    removed: FxHashSet<Coord>,
}
impl Overlay {
    fn clear(&mut self) {
        self.added.clear();
        self.removed.clear();
    }
}

#[derive(Default)]
struct Diff {
    changes: FxHashMap<Coord, Change>,
}
#[derive(Clone, Copy)]
struct Change {
    prev: Option<u8>,
    new: Option<u8>,
}

// --------------------------------------------------------------------------
// VoxelGrid – sparse chunk container
// --------------------------------------------------------------------------
pub struct VoxelGrid {
    chunks: FxHashMap<ChunkCoord, Chunk>, // *** only filled chunks ***
    preview: Overlay,                     // identical to previous
    undo_stack: Vec<Diff>,
    redo_stack: Vec<Diff>,

    bounds: [F; 3],
    size: [usize; 3],
    pub density: usize,
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new([6.0, 2.0, 4.0], 64)
    }
}

impl VoxelGrid {
    // ───────────────── construction ───────────────────────────────────────────
    pub fn new(bounds: [F; 3], density: usize) -> Self {
        let size = [
            (bounds[0] * density as F).ceil() as usize,
            (bounds[1] * density as F).ceil() as usize,
            (bounds[2] * density as F).ceil() as usize,
        ];

        let mut g = Self {
            chunks: FxHashMap::default(),
            preview: Overlay::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            bounds,
            size,
            density,
        };

        // 2-voxel floor ─ insert through the public API (writes into chunks)
        for z in 0..size[2] as i32 {
            for y in 0..2 {
                for x in 0..size[0] as i32 {
                    g.set(x, y, z, 100);
                }
            }
        }
        g
    }

    // ───────────────── helpers ────────────────────────────────────────────────
    #[inline]
    pub fn voxel_size(&self) -> Vec3<F> {
        Vec3::new(
            self.bounds[0] / self.size[0] as F,
            self.bounds[1] / self.size[1] as F,
            self.bounds[2] / self.size[2] as F,
        )
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

    /// Convert a world-space position to the **voxel corner index**
    /// (`None` if the point lies outside the grid AABB).
    #[inline]
    pub fn world_to_index(&self, p: Vec3<F>) -> Option<(i32, i32, i32)> {
        let shifted = p + Vec3::from(self.bounds) * 0.5;
        let vs = self.voxel_size();
        let v = (shifted / vs).map(|c| c.floor() as i32);

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
    fn bbox(&self) -> Aabb<F> {
        let h = Vec3::from(self.bounds) * 0.5;
        Aabb { min: -h, max: h }
    }

    #[inline]
    fn split(coord: Coord) -> (ChunkCoord, Coord) {
        let (x, y, z) = coord;
        let cx = x >> CHUNK_LG;
        let lx = x & CHUNK_MASK;
        let cy = y >> CHUNK_LG;
        let ly = y & CHUNK_MASK;
        let cz = z >> CHUNK_LG;
        let lz = z & CHUNK_MASK;
        ((cx, cy, cz), (lx, ly, lz))
    }

    // ───────────────── public fast get/set ────────────────────────────────────
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        let key = (x, y, z);
        if let Some(&m) = self.preview.added.get(&key) {
            return Some(m);
        }
        if self.preview.removed.contains(&key) {
            return None;
        }

        let (ck, local) = Self::split(key);
        self.chunks.get(&ck).and_then(|c| c.get(local))
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, mat: u8) -> Option<u8> {
        let (ck, local) = Self::split((x, y, z));
        let chunk = self.chunks.entry(ck).or_insert_with(Chunk::new);
        let prev = chunk.set(local, mat);

        if chunk.is_empty() {
            self.chunks.remove(&ck);
        }
        prev
    }

    #[inline]
    pub fn remove(&mut self, x: i32, y: i32, z: i32) -> Option<u8> {
        self.set(x, y, z, 0)
    }

    // ───────────────── preview / commit / undo identical logic ───────────────
    pub fn preview_add(&mut self, x: i32, y: i32, z: i32, m: u8) {
        self.preview.added.insert((x, y, z), m);
        self.preview.removed.remove(&(x, y, z));
    }
    pub fn preview_remove(&mut self, x: i32, y: i32, z: i32) {
        if self.get(x, y, z).is_some() {
            self.preview.removed.insert((x, y, z));
            self.preview.added.remove(&(x, y, z));
        }
    }
    pub fn clear_preview(&mut self) {
        self.preview.clear();
    }

    pub fn commit_preview(&mut self) {
        if self.preview.added.is_empty() && self.preview.removed.is_empty() {
            return;
        }

        let Overlay { added, removed } = mem::take(&mut self.preview);

        let mut diff = Diff::default();

        for (x, y, z) in removed {
            let prev = self.remove(x, y, z);
            diff.changes.insert((x, y, z), Change { prev, new: None });
        }

        for ((x, y, z), mat) in added {
            let prev = self.set(x, y, z, mat);
            diff.changes.insert(
                (x, y, z),
                Change {
                    prev,
                    new: Some(mat),
                },
            );
        }

        self.undo_stack.push(diff);
        self.redo_stack.clear();
    }

    fn pop_and_apply(&mut self, from: &mut Vec<Diff>, to: &mut Vec<Diff>) -> bool {
        if let Some(diff) = from.pop() {
            for (coord, change) in &diff.changes {
                match change.new {
                    Some(mat) => {
                        self.set(coord.0, coord.1, coord.2, mat);
                    }
                    None => {
                        self.remove(coord.0, coord.1, coord.2);
                    }
                }
            }
            to.push(diff);
            true
        } else {
            false
        }
    }

    pub fn undo(&mut self) -> bool {
        let diff = self.undo_stack.pop();
        if let Some(diff) = diff {
            for (coord, change) in &diff.changes {
                match change.prev {
                    Some(mat) => {
                        self.set(coord.0, coord.1, coord.2, mat);
                    }
                    None => {
                        self.remove(coord.0, coord.1, coord.2);
                    }
                }
            }
            self.redo_stack.push(diff);
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        let diff = self.redo_stack.pop();
        if let Some(diff) = diff {
            for (coord, change) in &diff.changes {
                match change.new {
                    Some(mat) => {
                        self.set(coord.0, coord.1, coord.2, mat);
                    }
                    None => {
                        self.remove(coord.0, coord.1, coord.2);
                    }
                }
            }
            self.undo_stack.push(diff);
            true
        } else {
            false
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
