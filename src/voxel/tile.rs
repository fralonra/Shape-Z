use crate::prelude::*;
use theframework::prelude::FxHashMap;

#[derive(Clone)]
pub struct Tile {
    pub voxels: FxHashMap<Coord, u8>,
    pub density: usize,
    pub bbox: Aabb<F>,
}

impl Tile {
    pub fn new(density: usize) -> Self {
        let mut voxels = FxHashMap::default();

        // Bottom floor
        for x in 0..density {
            for z in 0..density {
                voxels.insert((x as i32, 0, z as i32), 200);
            }
        }

        Self {
            voxels,
            density,
            bbox: Aabb {
                min: Vec3::zero(),
                max: Vec3::zero(),
            },
        }
    }

    pub fn update_bbox(&mut self) {
        let mut min = Vec3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Vec3::new(i32::MIN, i32::MIN, i32::MIN);

        for &(x, y, z) in self.voxels.keys() {
            min.x = min.x.min(x);
            min.y = min.y.min(y);
            min.z = min.z.min(z);

            max.x = max.x.max(x);
            max.y = max.y.max(y);
            max.z = max.z.max(z);
        }

        // Convert from voxel index space to float-local-space
        self.bbox = Aabb {
            min: min.map(|v| v as F),
            max: max.map(|v| v as F + 1.0), // +1 to make it exclusive on the upper side
        };
    }

    #[inline]
    pub fn get(&self, local: Coord) -> Option<u8> {
        self.voxels.get(&local).copied()
    }

    #[inline]
    pub fn set(&mut self, local: Coord, mat: u8) {
        self.voxels.insert(local, mat);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.voxels.is_empty()
    }

    pub fn dda(&self, ray: &Ray) -> Option<HitRecord> {
        let (_t_min, t_max) = ray.intersect_aabb(&self.bbox)?;

        let ro = ray.origin;
        let rd = ray.dir;

        #[inline(always)]
        fn equal(l: f32, r: Vec3<f32>) -> Vec3<f32> {
            r.map(|v| if l == v { 1.0 } else { 0.0 })
        }

        let mut i = ro.map(|v| v.floor());
        let srd = rd.map(|v| v.signum());
        let rdi = Vec3::broadcast(1.0) / (rd * 2.0);

        let mut normal = Vec3::zero();
        let max_steps = (self.density as f32 * 3.0).ceil() as i32;
        let mut t = 0.0;

        while t < t_max {
            let key = i.map(|v| v as i32);

            if let Some(material) = self.voxels.get(&(key.x, key.y, key.z)) {
                let hit = HitRecord {
                    hit: HitType::Voxel(*material),
                    hitpoint: ray.at(t),
                    distance: t,
                    normal,
                    local_key: (key.x, key.y, key.z),
                    ..Default::default()
                };

                return Some(hit);
            }

            let plane = (Vec3::broadcast(1.0) + srd - 2.0 * (ro - i)) * rdi;
            t = plane.x.min(plane.y.min(plane.z));
            normal = equal(t, plane) * srd;
            i += normal;
        }

        None
    }
}
