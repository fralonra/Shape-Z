use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {

    pub tiles               : FxHashMap<(i32, i32, i32), Tile>,
}

impl Project {
    pub fn new() -> Self {

        let mut tiles  = FxHashMap::default();

        tiles.insert((-1, 0, 0), Tile::new(49));
        tiles.insert((0, 0, 0), Tile::new(49));
        tiles.insert((1, 0, 0), Tile::new(49));

        Self {
            tiles,
        }
    }
}
