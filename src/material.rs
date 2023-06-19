use crate::prelude::*;
//use rhai::{ Engine, FuncArgs };
//use std::iter::once;

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub albedo          : Vec3f,
    pub roughness       : f32,
    pub metallic        : f32,
    pub reflectance     : f32,
    pub emission        : Vec3f,
    pub ior             : Option<f32>,
}

impl Material {
    pub fn new(albedo: Vec3f) -> Self {
        Self {
            albedo,
            roughness   : 0.5,
            metallic    : 0.5,
            reflectance : 1.0,
            emission    : Vec3f::zero(),
            ior         : None,
        }
    }

    /*
    pub fn get_hitpoint(&mut self) -> ScriptVec3f {
        ScriptVec3f::from_vec3f(self.hitpoint)
    }

    pub fn get_key(&mut self) -> ScriptVec3i {
        ScriptVec3i::from_vec3i(self.key)
    }

    pub fn get_tile_key(&mut self) -> ScriptVec3i {
        ScriptVec3i::from_vec3i(self.tile_key)
    }

    pub fn get_normal(&mut self) -> ScriptVec3f {
        ScriptVec3f::from_vec3f(self.normal)
    }

    pub fn get_value(&mut self) -> i32 {
        self.value as i32
    }

    pub fn get_side(&mut self) -> SideEnum {
        self.side.clone()
    }

    /// Register to the engine
    pub fn register(engine: &mut Engine) {
        engine.register_type_with_name::<HitRecord>("HitRecord")
            .register_get("hitpoint", HitRecord::get_hitpoint)
            .register_get("key", HitRecord::get_key)
            .register_get("tile_key", HitRecord::get_tile_key)
            .register_get("normal", HitRecord::get_normal)
            .register_get("value", HitRecord::get_value)
            .register_get("side", HitRecord::get_side);
    }*/
}
/*
impl FuncArgs for Material {
    fn parse<C: Extend<rhai::Dynamic>>(self, container: &mut C) {
        container.extend(once(rhai::Dynamic::from(self)));
    }
}*/