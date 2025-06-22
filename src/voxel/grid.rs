use theframework::prelude::FxHashMap;

use crate::F;
use vek::Aabb;
use vek::Vec3;

#[derive(Clone, Debug)]
pub struct VoxelGrid {
    voxels: FxHashMap<(i32, i32, i32), u8>, // sparse storage
    bounds: [F; 3],                         // world-space extents  (X, Y, Z)
    density: usize,                         // voxels per world unit
    size: [usize; 3],                       // derived voxel dims   (nx, ny, nz)
}

impl Default for VoxelGrid {
    /// Creates an empty 2×2×2 grid with density = 50 voxel per unit.
    fn default() -> Self {
        Self::new([2.0 as F, 2.0 as F, 2.0 as F], 64)
    }
}

impl VoxelGrid {
    /// Create an *empty* grid covering `bounds` (world units) with
    /// `density` voxels per unit.
    pub fn new(bounds: [F; 3], density: usize) -> Self {
        Self {
            voxels: FxHashMap::default(),
            size: Self::calc_size(bounds, density),
            bounds,
            density,
        }
    }

    /// Change voxel density (voxels per unit).
    /// **Warning:** existing voxels are cleared because their indices
    /// are no longer valid in the new resolution.
    pub fn set_density(&mut self, density: usize) {
        if density == 0 || density == self.density {
            return;
        }
        self.density = density;
        self.size = Self::calc_size(self.bounds, density);
        self.voxels.clear();
    }

    /// Change world-space bounds (in units).
    /// **Warning:** clears all stored voxels for the same reason as above.
    pub fn set_bounds(&mut self, bounds: [F; 3]) {
        self.bounds = bounds;
        self.size = Self::calc_size(bounds, self.density);
        self.voxels.clear();
    }

    #[inline]
    fn calc_size(bounds: [F; 3], density: usize) -> [usize; 3] {
        [
            (bounds[0] * density as F).ceil() as usize,
            (bounds[1] * density as F).ceil() as usize,
            (bounds[2] * density as F).ceil() as usize,
        ]
    }

    /*---------------------------------------------------------------------
    | basic sparse access
    *--------------------------------------------------------------------*/

    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        self.voxels.get(&(x, y, z)).copied()
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, value: u8) {
        self.voxels.insert((x, y, z), value);
    }

    pub fn clear(&mut self) {
        self.voxels.clear();
    }
    pub fn len(&self) -> usize {
        self.voxels.len()
    }
    pub fn is_empty(&self) -> bool {
        self.voxels.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&(i32, i32, i32), &u8)> {
        self.voxels.iter()
    }

    /// Convert voxel indices to world-space *cell centre*.
    /// Bottom-aligned Y, centred XZ (matches your dense `ModelBuffer`).
    #[inline]
    pub fn index_to_world(&self, x: i32, y: i32, z: i32) -> Vec3<F> {
        // single voxel size
        let vs = Vec3::new(
            self.bounds[0] / self.size[0] as F,
            self.bounds[1] / self.size[1] as F,
            self.bounds[2] / self.size[2] as F,
        );

        Vec3::new(
            x as F * vs.x - self.bounds[0] * 0.5 + vs.x * 0.5, // centre X
            y as F * vs.y + vs.y * 0.5,                        // centre Y
            z as F * vs.z - self.bounds[2] * 0.5 + vs.z * 0.5, // centre Z
        )
    }

    /// Convert world-space position to voxel indices.
    /// Returns `None` if outside the grid.
    #[inline]
    pub fn world_to_index(&self, p: Vec3<F>) -> Option<(i32, i32, i32)> {
        // shift to positive space (XZ centred, Y bottom-aligned)
        let shifted = Vec3::new(p.x + self.bounds[0] * 0.5, p.y, p.z + self.bounds[2] * 0.5);

        // scale to voxel coords
        let scale = Vec3::new(
            self.size[0] as F / self.bounds[0],
            self.size[1] as F / self.bounds[1],
            self.size[2] as F / self.bounds[2],
        );
        let v = shifted * scale;

        let (x, y, z) = (v.x.floor() as i32, v.y.floor() as i32, v.z.floor() as i32);

        if x >= 0
            && y >= 0
            && z >= 0
            && x < self.size[0] as i32
            && y < self.size[1] as i32
            && z < self.size[2] as i32
        {
            Some((x, y, z))
        } else {
            None
        }
    }

    /*---------------------------------------------------------------------
    | optional helpers (voxel size & bbox)
    *--------------------------------------------------------------------*/

    /// Average voxel edge length (world units).
    pub fn voxel_size(&self) -> F {
        let vs = Vec3::new(
            self.bounds[0] / self.size[0] as F,
            self.bounds[1] / self.size[1] as F,
            self.bounds[2] / self.size[2] as F,
        );
        (vs.x + vs.y + vs.z) / 3.0
    }

    /// Axis-aligned bounding box centred in XZ, bottom-aligned in Y.
    pub fn bbox(&self) -> Aabb<F> {
        Aabb {
            min: Vec3::new(-self.bounds[0] * 0.5, 0.0, -self.bounds[2] * 0.5),
            max: Vec3::new(self.bounds[0] * 0.5, self.bounds[1], self.bounds[2] * 0.5),
        }
    }

    /// Adds a sphere at the given position
    pub fn add_sphere(&mut self, center: Vec3<F>, radius: F, value: u8) {
        let r2 = radius * radius;

        // Compute bounding box in voxel index space
        let min = center - Vec3::broadcast(radius);
        let max = center + Vec3::broadcast(radius);

        let min_ix = self.world_to_index(min).unwrap_or((0, 0, 0));
        let max_ix = self.world_to_index(max).unwrap_or((
            self.size[0] as i32 - 1,
            self.size[1] as i32 - 1,
            self.size[2] as i32 - 1,
        ));

        for z in min_ix.2..=max_ix.2 {
            for y in min_ix.1..=max_ix.1 {
                for x in min_ix.0..=max_ix.0 {
                    let world = self.index_to_world(x, y, z);
                    let dist2 = (world - center).magnitude_squared();

                    if dist2 <= r2 {
                        self.set(x, y, z, value);
                    }
                }
            }
        }
    }
}
