use crate::prelude::*;
use theframework::prelude::FxHashMap;

#[derive(Clone)]
pub struct VoxelGrid {
    pub tiles: FxHashMap<Coord, Tile>,
    pub density: usize,
    pub density_f: F,
    pub bounds: [F; 3],
}

impl Default for VoxelGrid {
    fn default() -> Self {
        Self::new([2.0, 2.0, 2.0], 128)
    }
}

impl VoxelGrid {
    pub fn new(bounds: [F; 3], density: usize) -> Self {
        let mut tiles = FxHashMap::default();

        let x_tiles = bounds[0].ceil() as i32;
        let y_tiles = bounds[1].ceil() as i32;
        let z_tiles = bounds[2].ceil() as i32;

        let x_start = -x_tiles / 2;
        let y_start = -y_tiles / 2;
        let z_start = -z_tiles / 2;

        for tz in 0..z_tiles {
            for ty in 0..y_tiles {
                for tx in 0..x_tiles {
                    let mut tile = Tile::new(density);
                    tile.update_bbox();
                    tiles.insert((x_start + tx, y_start + ty, z_start + tz), tile);
                }
            }
        }

        Self {
            tiles,
            density,
            density_f: density as F,
            bounds,
        }
    }

    /// Get a voxel at the given world coordinate
    #[inline(always)]
    pub fn get(&self, wc: Vec3<f32>) -> Option<u8> {
        let (tile_key, local_key) = self.to_tile_coord(wc);
        self.tiles.get(&tile_key)?.get(local_key)
    }

    /// Set a voxel at the given world coordinate
    #[inline(always)]
    pub fn set(&mut self, wc: Vec3<f32>, mat: u8) {
        let (tile_key, local_key) = self.to_tile_coord(wc);
        if let Some(tile) = self.tiles.get_mut(&tile_key) {
            tile.set(local_key, mat);
        }
    }

    /// Set a voxel at the given world coordinate and create a new tile if necessary.
    #[inline(always)]
    pub fn set_create(&mut self, wc: Vec3<f32>, mat: u8) {
        let (tile_key, local_key) = self.to_tile_coord(wc);
        self.tiles
            .entry(tile_key)
            .or_insert_with(|| Tile::new(self.density))
            .set(local_key, mat);
    }

    /// Converts the hit keys to a world coordinate
    #[inline(always)]
    pub fn to_world_coord(&self, tile: Coord, local: Coord) -> Vec3<f32> {
        let mut wc = Vec3::new(tile.0 as f32, tile.1 as f32, tile.2 as f32);

        wc.x += local.0 as f32 / self.density_f;
        wc.y += local.1 as f32 / self.density_f;
        wc.z += local.2 as f32 / self.density_f;

        wc
    }

    /// Converts the world coordinate to hit keys
    #[inline(always)]
    pub fn to_tile_coord(&self, wc: Vec3<f32>) -> (Coord, Coord) {
        let tile = (
            wc.x.floor() as i32,
            wc.y.floor() as i32,
            wc.z.floor() as i32,
        );
        let local = (
            ((wc.x - tile.0 as f32) * self.density_f).floor() as i32,
            ((wc.y - tile.1 as f32) * self.density_f).floor() as i32,
            ((wc.z - tile.2 as f32) * self.density_f).floor() as i32,
        );
        (tile, local)
    }

    /// World-space Aabb of the whole grid
    #[inline]
    fn bbox(&self) -> Aabb<F> {
        let h = Vec3::from(self.bounds) * 0.5;
        Aabb { min: -h, max: h }
    }

    /// Recursively dda the tiles
    pub fn dda(&self, ray: &Ray) -> HitRecord {
        // Port of https://www.shadertoy.com/view/ct33Rn

        #[inline(always)]
        fn equal(l: f32, r: Vec3<f32>) -> Vec3<f32> {
            r.map(|v| if l == v { 1.0 } else { 0.0 })
        }

        let (t_min, t_max) = match ray.intersect_aabb(&self.bbox()) {
            Some(b) => b,
            None => return HitRecord::default(),
        };

        // let t_min = 0.0_f32;
        // let t_max = 10.0_f32;

        let mut t = t_min.max(0.0);

        let ro = ray.origin; //at(t);
        let rd = ray.dir;

        let mut i = ro.map(|v| v.floor());
        let srd = rd.map(|v| v.signum());
        let rdi = Vec3::broadcast(1.0) / (rd * 2.0);

        while t < t_max {
            let key = i.map(|v| v as i32);

            if let Some(tile) = self.tiles.get(&(key.x, key.y, key.z)) {
                let mut lro = ray.at(t);
                lro -= i; // subtract tile origin
                lro *= tile.density as f32; // scale to voxel grid
                lro -= rd * 0.01; // tiny epsilon

                if !tile.is_empty() {
                    // Cast inside the tile’s dense voxel grid
                    if let Some(mut hit) = tile.dda(&Ray::new(lro, rd)) {
                        hit.tile_key = (key.x, key.y, key.z);
                        hit.hitpoint = ray.at(t + hit.distance / self.density_f);
                        hit.distance = t;
                        return hit;
                    }
                }
            }

            let plane = (Vec3::broadcast(1.0) + srd - 2.0 * (ro - i)) * rdi;
            t = plane.x.min(plane.y.min(plane.z));
            let normal = equal(t, plane) * srd;
            i += normal;
        }

        HitRecord {
            hit: HitType::BBox((t_min, t_max)),
            ..Default::default()
        }
    }
}
