
fn init() {

    this.name = "Remove";
    this.role = "Voxel";
}

fn apply(key) {

    let tile = get_tile(key);
    tile.set(vec3i(5, 5, 5), 10);
    set_tile(key, tile);
}

fn hit(record) {
    let tile = get_tile(record.key);
    tile.clear_at(record.tile_key);
    set_tile(record.key, tile);
}