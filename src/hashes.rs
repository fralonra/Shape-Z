use crate::prelude::*;

/*
vec2 hash2(vec3 p3) {
	p3 = fract(p3 * vec3(5.3983, 5.4427, 6.9371));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.xx + p3.yz) * p3.zy);
}*/

pub fn hash3_2(mut p3: Vec3f) -> Vec2f {
    p3 = frac(p3 * Vec3f::new(5.3983, 5.4427, 6.9371 ));
    p3 += dot(p3, p3.yzx() + 19.19);
    frac((p3.xx() + p3.yz()) * p3.zy())
}