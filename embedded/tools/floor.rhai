
fn init() {

    this.name = "Floor";
    this.role = "Voxel";

    this.parameters = [
        color("Color", 5),
        material("Material", 0)
    ];
}

fn apply(key) {
    let tile = get_tile(key);
    tile.set(vec3i(5, 5, 5), 10);
    set_tile(key, tile);
}

fn hit(record) {
    let tile = get_tile(record.key);

    let size = tile.size;

    let start_y = record.tile_key.y + 1;
    let height = 1;

    for y in 0..height {
        for x in 0..size {
            for z in 0..size {
                if random() > 0.5 {
                    tile.set(vec3i(x, start_y + y, z), "Color", "Material");
                }
            }
        }
    }

    set_tile(record.key, tile);
}