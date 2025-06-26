use crate::prelude::*;
use std::mem;
use theframework::prelude::{FxHashMap, FxHashSet};
use vek::{Aabb, Vec3};

pub type VoxelCoord = (i32, i32, i32); // global voxel key (world–space index)
pub type ChunkCoord = (i32, i32, i32); // integer-world cube key (1 unit³)

// ───────────────────────────── Chunk ────────────────────────────────────────
#[derive(Clone)]
struct Chunk {
    vox: FxHashMap<VoxelCoord, u8>, // local key ∈ [0..density)
    density: usize,                 // voxels per axis inside this 1 × 1 × 1 cube
}

impl Chunk {
    fn new(density: usize) -> Self {
        Self {
            vox: FxHashMap::default(),
            density,
        }
    }

    #[inline]
    fn voxel_size(&self) -> f32 {
        1.0 / self.density as f32
    }

    #[inline]
    fn get(&self, local: VoxelCoord) -> Option<u8> {
        self.vox.get(&local).copied()
    }
    #[inline]
    fn set(&mut self, local: VoxelCoord, mat: u8) -> Option<u8> {
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

// ────────────────────────── Overlay / Diff ─────────────────────────────────
#[derive(Default)]
struct Overlay {
    added: FxHashMap<VoxelCoord, u8>,
    removed: FxHashSet<VoxelCoord>,
}
impl Overlay {
    fn clear(&mut self) {
        self.added.clear();
        self.removed.clear();
    }
}

#[derive(Default)]
struct Diff {
    changes: FxHashMap<VoxelCoord, Change>,
}
#[derive(Clone, Copy)]
struct Change {
    prev: Option<u8>,
    new: Option<u8>,
}

// ─────────────────────────── VoxelGrid ─────────────────────────────────────
pub struct VoxelGrid {
    chunks: FxHashMap<ChunkCoord, Chunk>, // sparse
    preview: Overlay,
    undo_stack: Vec<Diff>,
    redo_stack: Vec<Diff>,

    bounds: [F; 3],     // world extent (for clamping)
    pub density: usize, // default density for *new* chunks
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new([2.0, 2.0, 2.0], 128)
    }
}

impl VoxelGrid {
    // ─────────── construction ───────────
    pub fn new(bounds: [F; 3], default_density: usize) -> Self {
        let mut g = Self {
            chunks: FxHashMap::default(),
            preview: Overlay::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            bounds,
            density: default_density,
        };

        // ───────────────────────
        // 2-voxel-thick floor
        // ───────────────────────
        let density = default_density;
        let voxel_size = 1.0 / density as F;
        let floor_height = 2.0 * voxel_size;

        let x_chunks = (bounds[0].ceil() as i32);
        let z_chunks = (bounds[2].ceil() as i32);

        for cz in 0..z_chunks {
            for cx in 0..x_chunks {
                let chunk_coord = (cx, 0, cz);
                let mut chunk = Chunk::new(density);

                for y in 0..2 {
                    for z in 0..density {
                        for x in 0..density {
                            chunk.set((x as i32, y, z as i32), 100); // material 100 for floor
                        }
                    }
                }

                g.chunks.insert(chunk_coord, chunk);
            }
        }

        g
    }

    // ─────────── helpers ────────────────
    /// world-space AABB of the whole grid
    #[inline]
    fn bbox(&self) -> Aabb<F> {
        let h = Vec3::from(self.bounds) * 0.5;
        Aabb { min: -h, max: h }
    }

    /// Chunk that contains the point *p* and the local (fractional) position inside it
    #[inline]
    fn world_to_chunk(&self, p: Vec3<F>) -> (ChunkCoord, Vec3<F>) {
        // shift so that (0,0,0) is at grid min corner
        let shifted = p + Vec3::from(self.bounds) * 0.5;
        let c = shifted.map(|v| v.floor() as i32); // chunk integer coords
        let frac = shifted - c.map(|i| i as f32); // [0,1) inside chunk
        ((c.x, c.y, c.z), frac)
    }

    /// Convert (chunk coord, *local fractional*) into a *local voxel index* for that chunk
    #[inline]
    fn frac_to_local(chunk: &Chunk, frac: Vec3<F>) -> VoxelCoord {
        let f = frac * chunk.density as f32; // 0 … density
        (f.x.floor() as i32, f.y.floor() as i32, f.z.floor() as i32)
    }

    // ─────────── fast get/set via world-space position ───────────
    #[inline]
    pub fn get_world(&self, p: Vec3<F>) -> Option<u8> {
        let (ck, frac) = self.world_to_chunk(p);
        let chunk = self.chunks.get(&ck)?;
        let local = Self::frac_to_local(chunk, frac);
        // overlay first
        if let Some(m) = self.preview.added.get(&(
            local.0 + ck.0 * chunk.density as i32,
            local.1 + ck.1 * chunk.density as i32,
            local.2 + ck.2 * chunk.density as i32,
        )) {
            return Some(*m);
        }
        if self.preview.removed.contains(&local) {
            return None;
        }
        chunk.get(local)
    }

    #[inline]
    pub fn set_world(
        &mut self,
        p: Vec3<F>,
        mat: u8,
        density_override: Option<usize>,
    ) -> Option<u8> {
        let (ck, frac) = self.world_to_chunk(p);
        let default_d = self.density;
        let chunk = self
            .chunks
            .entry(ck)
            .or_insert_with(|| Chunk::new(density_override.unwrap_or(default_d)));

        // If the caller provided an override for an *existing* chunk, honour it once
        if let Some(d) = density_override {
            chunk.density = d.max(1);
        }

        let local = Self::frac_to_local(chunk, frac);
        let prev = chunk.set(local, mat);
        if chunk.is_empty() {
            self.chunks.remove(&ck);
        }
        prev
    }

    // convenience wrappers that keep your old integer signature if you want them
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, mat: u8) -> Option<u8> {
        let p = self.index_to_world(x, y, z);
        self.set_world(p, mat, None)
    }
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        let p = self.index_to_world(x, y, z);
        self.get_world(p)
    }

    /// Convert back to world point (corner of voxel (ix,iy,iz) at *default* density)
    #[inline]
    fn index_to_world(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        let vs = 1.0 / self.density as f32;
        (Vec3::new(ix as F, iy as F, iz as F) * vs) - Vec3::from(self.bounds) * 0.5
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

    // ───────────── two-stage DDA ─────────────
    ///
    ///  1.  Walk **chunk grid** (step = 1 world unit) → find first non-empty chunk
    ///  2.  Inside that chunk run classic voxel-level DDA at its own resolution
    ///
    pub fn dda(&self, ray: &Ray) -> HitRecord {
        // ── 1. intersect whole grid ───────────────────────────
        let (mut t_min, t_max) = match ray.intersect_aabb(&self.bbox()) {
            Some(b) => b,
            None => return HitRecord::default(),
        };
        if t_min < 0.0 {
            t_min = 0.0;
        }

        // prepare chunk stepping (chunk cell = 1 unit)
        let eps = 1e-4;
        let mut p = ray.at(&(t_min + eps));
        let (mut ck, mut frac) = self.world_to_chunk(p);

        // dir sign, t_delta per chunk, t_max for first boundary
        let step_c = ray.dir.map(|d| if d < 0.0 { -1 } else { 1 });
        let inv_dir = ray.dir.map(|d| 1.0 / d.abs().max(1e-30));
        let t_delta_c: Vec3<f32> = Vec3::one() * inv_dir;
        let mut t_max_c = Vec3::zero();
        let half_bounds = Vec3::from(self.bounds) * 0.5;

        // X axis
        let bound_x = if step_c.x > 0 {
            ck.0 as f32 + 1.0
        } else {
            ck.0 as f32
        };
        t_max_c.x = (bound_x - (p.x + half_bounds.x)) * inv_dir.x;

        // Y axis
        let bound_y = if step_c.y > 0 {
            ck.1 as f32 + 1.0
        } else {
            ck.1 as f32
        };
        t_max_c.y = (bound_y - (p.y + half_bounds.y)) * inv_dir.y;

        // Z axis
        let bound_z = if step_c.z > 0 {
            ck.2 as f32 + 1.0
        } else {
            ck.2 as f32
        };
        t_max_c.z = (bound_z - (p.z + half_bounds.z)) * inv_dir.z;

        let t_end = t_max.min(1_000.0);
        let mut t = t_min + eps;

        // ── walk chunks ───────────────────────────────────────
        loop {
            if t > t_end {
                break;
            }

            if let Some(chunk) = self.chunks.get(&ck) {
                // ── 2. inside this chunk: do fine DDA ─────────
                if let Some(hit) = self.dda_inside_chunk(chunk, ck, ray, t) {
                    return hit;
                }
            }

            // advance to next chunk
            if t_max_c.x < t_max_c.y {
                if t_max_c.x < t_max_c.z {
                    ck.0 += step_c.x;
                    t = t_max_c.x;
                    t_max_c.x += t_delta_c.x;
                } else {
                    ck.2 += step_c.z;
                    t = t_max_c.z;
                    t_max_c.z += t_delta_c.z;
                }
            } else if t_max_c.y < t_max_c.z {
                ck.1 += step_c.y;
                t = t_max_c.y;
                t_max_c.y += t_delta_c.y;
            } else {
                ck.2 += step_c.z;
                t = t_max_c.z;
                t_max_c.z += t_delta_c.z;
            }
            // hit nothing in this chunk → continue
        }

        HitRecord {
            hit: HitType::BBox((t_min, t_max)),
            ..Default::default()
        }
    }

    /// Classic voxel-accurate DDA inside a single 1 × 1 × 1 chunk.
    ///
    /// * `chunk` – the chunk we are inside (has its own `density`)
    /// * `ck`    – integer chunk coordinate (world-space)
    /// * `ray`   – incoming ray (in world units, already known to intersect this chunk)
    /// * `t_start` – parametric distance where the ray first touched the chunk
    ///
    /// Returns `Some(HitRecord)` on the first voxel hit, otherwise `None`.
    fn dda_inside_chunk(
        &self,
        chunk: &Chunk,
        ck: ChunkCoord,
        ray: &Ray,
        t_start: F,
    ) -> Option<HitRecord> {
        // ───── basic data ────────────────────────────────────────────────────────
        let density = chunk.density as i32;
        let vs = 1.0 / density as F; // voxel size inside this chunk

        // world-space AABB for this chunk
        let chunk_min = Vec3::new(ck.0 as F, ck.1 as F, ck.2 as F) - Vec3::from(self.bounds) * 0.5;
        let chunk_max = chunk_min + Vec3::one();
        let chunk_box = Aabb {
            min: chunk_min,
            max: chunk_max,
        };

        // t0 / t1 inside this chunk (we already know it intersects)
        let (mut t0, t1) = ray.intersect_aabb(&chunk_box)?;
        if t0 < t_start {
            t0 = t_start;
        }

        // ───── initial voxel index ───────────────────────────────────────────────
        let eps = 1e-4;
        let p_local = ray.at(&(t0 + eps)) - chunk_min; // [0,1) position inside chunk

        let mut ix = (p_local.x / vs).floor() as i32;
        let mut iy = (p_local.y / vs).floor() as i32;
        let mut iz = (p_local.z / vs).floor() as i32;

        // guard (can happen due to precision)
        if ix < 0 || iy < 0 || iz < 0 || ix >= density || iy >= density || iz >= density {
            return None;
        }

        // ───── DDA coefficients ─────────────────────────────────────────────────
        let step = ray.dir.map(|d| if d < 0.0 { -1 } else { 1 });
        let abs_d = ray.dir.map(|d| d.abs().max(1e-30));
        let t_delta = Vec3::new(vs / abs_d.x, vs / abs_d.y, vs / abs_d.z);

        let mut t_max = Vec3::zero();

        // helper to compute distance to next voxel boundary per axis
        macro_rules! next_boundary {
            ($pos:expr, $i:expr, $step:expr) => {
                if $step > 0 {
                    (($i + 1) as F * vs) - $pos
                } else {
                    ($i as F * vs) - $pos
                }
            };
        }

        t_max.x = t0 + next_boundary!(p_local.x, ix, step.x) / abs_d.x;
        t_max.y = t0 + next_boundary!(p_local.y, iy, step.y) / abs_d.y;
        t_max.z = t0 + next_boundary!(p_local.z, iz, step.z) / abs_d.z;

        // ───── traversal loop ───────────────────────────────────────────────────
        let mut t = t0;
        while t <= t1 {
            // hit?
            if let Some(mat) = chunk.get((ix, iy, iz)) {
                // convert local voxel index to a *global* one (for compatibility)
                let gx = ix + ck.0 * density;
                let gy = iy + ck.1 * density;
                let gz = iz + ck.2 * density;

                // centre of this voxel (world space)
                let centre = chunk_min + (Vec3::new(ix as F, iy as F, iz as F) + 0.5) * vs;
                let hit_world = ray.at(&t);
                let delta = hit_world - centre;

                let (normal, face) =
                    if delta.x.abs() > delta.y.abs() && delta.x.abs() > delta.z.abs() {
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

                return Some(HitRecord {
                    hit_point_local: Vec3::new(gx, gy, gz),
                    hit_point_world: hit_world,
                    normal,
                    face,
                    hit: HitType::Voxel(mat),
                });
            }

            // advance to next voxel
            if t_max.x < t_max.y {
                if t_max.x < t_max.z {
                    ix += step.x;
                    t = t_max.x;
                    t_max.x += t_delta.x;
                } else {
                    iz += step.z;
                    t = t_max.z;
                    t_max.z += t_delta.z;
                }
            } else if t_max.y < t_max.z {
                iy += step.y;
                t = t_max.y;
                t_max.y += t_delta.y;
            } else {
                iz += step.z;
                t = t_max.z;
                t_max.z += t_delta.z;
            }

            // bounds test inside current chunk
            if ix < 0 || iy < 0 || iz < 0 || ix >= density || iy >= density || iz >= density {
                break; // exited the chunk
            }
        }

        None
    }
}

/*
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
    pub voxel_size: Vec3<F>,
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new([2.0, 2.0, 2.0], 128)
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

        let voxel_size = Vec3::new(
            bounds[0] / size[0] as F,
            bounds[1] / size[1] as F,
            bounds[2] / size[2] as F,
        );

        let mut g = Self {
            chunks: FxHashMap::default(),
            preview: Overlay::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            bounds,
            size,
            density,
            voxel_size,
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

    /// world **corner** of voxel `(ix,iy,iz)`
    #[inline]
    fn index_to_corner(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        (Vec3::new(ix as F, iy as F, iz as F) * self.voxel_size) - Vec3::from(self.bounds) * 0.5
    }

    /// world **centre** of voxel `(ix,iy,iz)`
    #[inline]
    fn index_to_centre(&self, ix: i32, iy: i32, iz: i32) -> Vec3<F> {
        self.index_to_corner(ix, iy, iz) + self.voxel_size * 0.5
    }

    /// Convert a world-space position to the **voxel corner index**
    /// (`None` if the point lies outside the grid AABB).
    #[inline]
    pub fn world_to_index(&self, p: Vec3<F>) -> Option<(i32, i32, i32)> {
        let shifted = p + Vec3::from(self.bounds) * 0.5;
        let vs = self.voxel_size;
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
        let vs = self.voxel_size;
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
*/
