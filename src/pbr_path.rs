use crate::prelude::*;

pub fn jitter(d: Vec3f, phi: f32, sina: f32, cosa: f32) -> Vec3f {
	let w = normalize(d);
    let u = normalize(cross(w.yzx(), w));
    let v = cross(w, u);
	return (u*cos(phi) + v*sin(phi)) * sina + w * cosa;
}

pub fn angle_to_dir(n: Vec3f, theta: f32, phi: f32) -> Vec3f {
    let sin_phi = sin(phi);
    let cos_phi = cos(phi);
    let w = normalize(n);
    let u = normalize(cross(w.yzx(), w));
    let v = cross(w, u);
    return (u * cos(theta) + v * sin(theta)) * sin_phi + w * cos_phi;
}

/*
float ggx(vec3 N, vec3 V, vec3 L, float roughness, float F0)
*/

#[allow(non_snake_case)]
pub fn ggx(N: Vec3f, V: Vec3f, L: Vec3f, roughness: f32, F0: f32) -> f32 {

    #[inline(always)]
    pub fn mix(a: &f32, b: &f32, v: f32) -> f32 {
        (1.0 - v) * a + b * v
    }

    let H = normalize(V + L);

    let dotLH = max(dot(L, H), 0.0);
    let dotNH = max(dot(N, H), 0.0);
    let dotNL = max(dot(N, L), 0.0);
    let dotNV = max(dot(N, V), 0.0);

    let alpha = roughness * roughness + 0.0001;

    // GGX normal distribution
    let alphaSqr = alpha * alpha;
    let denom = dotNH * dotNH * (alphaSqr - 1.0) + 1.0;
    let D = alphaSqr / (denom * denom);

    // Fresnel term approximation
    let F_a = 1.0;
    let F_b = powf(1.0 - dotLH, 5.0);
    let F = mix(&F_b, &F_a, F0);

    // GGX self shadowing term
    let k = (alpha + 2.0 * roughness + 1.0) / 8.0;
    let G = dotNL / (mix(&dotNL, &1.0, k) * mix(&dotNV, &1.0, k));

    // '* dotNV' - Is canceled due to normalization
    // '/ dotLN' - Is canceled due to lambert
    // '/ dotNV' - Is canceled due to G
    return max(0.0, min(10.0, D * F * G / 4.0));
}