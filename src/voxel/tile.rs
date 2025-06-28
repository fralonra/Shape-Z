use crate::prelude::*;

#[derive(Clone)]
pub struct Tile {
    pub voxels: Vec<Option<u8>>,
    pub density: usize,
    pub bbox: Aabb<F>,
    pub has_voxels: bool,
}

impl Tile {
    pub fn new(density: usize) -> Self {
        let total = density * density * density;

        Self {
            voxels: vec![None; total],
            density,
            bbox: Aabb {
                min: Vec3::zero(),
                max: Vec3::zero(),
            },
            has_voxels: false,
        }
    }

    pub fn add_floor(&mut self) {
        let max_index = 15.0;

        let size = self.density as f32;

        for x in 0..self.density {
            for z in 0..self.density {
                let xf = x as f32;
                let zf = z as f32;

                // Distance to nearest edge
                let dx = xf.min(size - 1.0 - xf);
                let dz = zf.min(size - 1.0 - zf);
                let d = dx.min(dz);

                // Normalize distance to [0.0, 1.0]
                let max_dist = (size - 1.0) / 2.0;
                let norm = (1.0 - d / max_dist).clamp(0.0, 1.0);

                // Spread index linearly from border (0) to center (15)
                let index = (norm * max_index).round() as u8;

                self.set((x as i32, 0, z as i32), index);
            }
        }

        self.update_bbox();
    }

    pub fn update_bbox(&mut self) {
        let mut min = Vec3::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Vec3::new(i32::MIN, i32::MIN, i32::MIN);
        let d = self.density as i32;

        let mut found = false;

        for z in 0..d {
            for y in 0..d {
                for x in 0..d {
                    if self.get((x, y, z)).is_some() {
                        found = true;
                        min.x = min.x.min(x);
                        min.y = min.y.min(y);
                        min.z = min.z.min(z);
                        max.x = max.x.max(x);
                        max.y = max.y.max(y);
                        max.z = max.z.max(z);
                    }
                }
            }
        }

        self.has_voxels = found;

        if found {
            self.bbox = Aabb {
                min: min.map(|v| v as F),
                max: max.map(|v| v as F + 1.0),
            };
        } else {
            self.bbox = Aabb {
                min: Vec3::zero(),
                max: Vec3::zero(),
            };
        }
    }

    #[inline(always)]
    fn index(&self, (x, y, z): Coord) -> Option<usize> {
        if x >= 0 && y >= 0 && z >= 0 {
            let d = self.density as i32;
            if x < d && y < d && z < d {
                let i = (z * d * d + y * d + x) as usize;
                return Some(i);
            }
        }
        None
    }

    #[inline]
    pub fn get(&self, coord: Coord) -> Option<u8> {
        self.index(coord).and_then(|i| self.voxels[i])
    }

    #[inline]
    pub fn set(&mut self, coord: Coord, mat: u8) {
        if let Some(i) = self.index(coord) {
            self.voxels[i] = Some(mat);
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.has_voxels
    }

    pub fn dda(&self, ray: &Ray) -> Option<HitRecord> {
        let (_, t_max) = ray.intersect_aabb(&self.bbox)?;

        let mut t = 0.0; //t_min;
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

        while t < t_max {
            let key = i.map(|v| v as i32);

            if let Some(material) = self.get((key.x, key.y, key.z)) {
                return Some(HitRecord {
                    hit: HitType::Voxel(material),
                    hitpoint: ray.at(t),
                    distance: t,
                    normal,
                    local_key: (key.x, key.y, key.z),
                    ..Default::default()
                });
            }

            let plane = (Vec3::broadcast(1.0) + srd - 2.0 * (ro - i)) * rdi;
            t = plane.x.min(plane.y.min(plane.z));
            normal = equal(t, plane) * srd;
            i += normal;
        }

        None
    }
}
