use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {

    pub tiles               : FxHashMap<(i32, i32, i32), Tile>,

    pub aabb                : Option<AABB>,
}

impl Project {
    pub fn new() -> Self {

        let mut tiles  = FxHashMap::default();

        let mut tile = Tile::new(Project::tile_size());
        tile.build_aabb();

        tiles.insert((-1, 0, 0), tile.clone());
        tiles.insert((0, 0, 0), tile.clone());
        tiles.insert((1, 0, 0), tile);

        Self {
            tiles,
            aabb            : None
        }
    }

    /// Build an aaab for the tiles voxels
    pub fn build_aabb(&mut self) {

        if self.tiles.is_empty() {
            self.aabb = None;
            return;
        }

        let mut min = Vec3f::new(core::f32::MAX, core::f32::MAX, core::f32::MAX);
        let mut max = Vec3f::new(core::f32::MIN, core::f32::MIN, core::f32::MIN);

        for (x, y, z) in self.tiles.keys() {

            let x_f = *x as f32;
            let y_f = *y as f32;
            let z_f = *z as f32;

            if x_f < min.x {
                min.x = x_f;
            }
            if x_f >= max.x {
                max.x = x_f + 1.0;
            }

            if y_f < min.y {
                min.y = x_f;
            }
            if y_f >= max.y {
                max.y = y_f + 1.0;
            }

            if z_f < min.z {
                min.z = z_f;
            }
            if z_f >= max.z {
                max.z = z_f + 1.0;
            }
        }

        self.aabb = Some(AABB { min, max } );
        //println!("{:?}", self.aabb);
    }

    /// Ray AABB intersection. Taken from https://github.com/svenstaro/bvh/blob/master/src/ray.rs
    pub fn ray_aabb(&self, ray: &Ray, aabb: &AABB) -> bool {
        let mut ray_min = (aabb[ray.sign_x].x - ray.o.x) * ray.inv_direction.x;
        let mut ray_max = (aabb[1 - ray.sign_x].x - ray.o.x) * ray.inv_direction.x;

        let y_min = (aabb[ray.sign_y].y - ray.o.y) * ray.inv_direction.y;
        let y_max = (aabb[1 - ray.sign_y].y - ray.o.y) * ray.inv_direction.y;

        ray_min = max(ray_min, y_min);
        ray_max = min(ray_max, y_max);

        let z_min = (aabb[ray.sign_z].z - ray.o.z) * ray.inv_direction.z;
        let z_max = (aabb[1 - ray.sign_z].z - ray.o.z) * ray.inv_direction.z;

        ray_min = max(ray_min, z_min);
        ray_max = min(ray_max, z_max);

        max(ray_min, 0.0) <= ray_max
    }

    /// The project tile size
    pub fn tile_size() -> usize {
        100
    }
}
