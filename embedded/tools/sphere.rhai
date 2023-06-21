
fn init() {

    this.name = "Sphere";
    this.role = "Procedural";

    this.parameters = [
        color("Color", 5),
        material("Material", 10)
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

    let hp = vec3f(record.tile_key.x.to_float(), record.tile_key.y.to_float(), record.tile_key.z.to_float());

    for y in 0..size {
        for x in 0..size {
            for z in 0..size {
                let p = vec3f(x.to_float(), y.to_float(), z.to_float());
                if length(p - hp) - size.to_float() / 3.0 < 0.0 {
                    tile.set(vec3i(x, y, z), "Color", "Material");
                }
            }
        }
    }

    set_tile(record.key, tile);
}