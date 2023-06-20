
fn init() {

    this.name = "Clear";
    this.role = "Tile";

    this.parameters = [
    ];
}

fn apply(key) {
    let tile = get_tile(key);
    tile.set(vec3i(5, 5, 5), 10);
    set_tile(key, tile);
}

fn hit(record) {
    let tile = get_tile(record.key);
    tile.clear_all();
    set_tile(record.key, tile);
}