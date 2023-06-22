
fn init() {

    this.name = "Floor";
    this.role = "Voxel";

    this.parameters = [
        color("Color", 5),
        material("Material", 0)
    ];
}

fn apply(key) {
    do_floor(key, 0);
}

fn hit(record) {
    let start_y = record.tile_key.y + 1;
    do_floor(record.key, start_y);
}

fn do_floor(key, start_y) {
    let tile = get_tile(key);

    let size = tile.size;

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

    set_tile(key, tile);
}