use crate::prelude::*;
use rhai::{ Engine, FuncArgs };
use std::iter::once;

pub fn setup_engine() -> Engine {
    let mut engine = Engine::new();

    ScriptVec3i::register(&mut engine);
    ScriptVec3f::register(&mut engine);

    ScriptValue::register(&mut engine);

    engine
}

/// ScriptVec3i
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ScriptVec3i {
    pub v                   : Vec3i,
}

impl ScriptVec3i {

    pub fn from(v: ScriptVec3i) -> Self {
        Self {
            v               : v.v,
        }
    }

    pub fn from_vec3i(v: Vec3i) -> Self {
        Self {
            v,
        }
    }

    pub fn zeros() -> Self {
        Self {
            v               : Vec3i::zero(),
        }
    }

    pub fn new_x(v: i32) -> Self {
        Self {
            v               : splat3i(v),
        }
    }

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            v               : vec3i(x, y, z),
        }
    }

    pub fn get_x(&mut self) -> i32 {
        self.v.x
    }

    pub fn set_x(&mut self, new_val: i32) {
        self.v.x = new_val;
    }

    pub fn get_y(&mut self) -> i32 {
        self.v.y
    }

    pub fn set_y(&mut self, new_val: i32) {
        self.v.y = new_val;
    }

    pub fn get_z(&mut self) -> i32 {
        self.v.z
    }

    pub fn set_z(&mut self, new_val: i32) {
        self.v.z = new_val;
    }

    /// Register to the engine
    pub fn register(engine: &mut Engine) {
        engine.register_type_with_name::<ScriptVec3i>("vec3i")
            .register_fn("vec3i", ScriptVec3i::zeros)
            .register_fn("vec3i", ScriptVec3i::new)
            .register_fn("vec3i", ScriptVec3i::new_x)
            .register_fn("vec3i", ScriptVec3i::from)
            .register_get_set("x", ScriptVec3i::get_x, ScriptVec3i::set_x)
            .register_get_set("y", ScriptVec3i::get_y, ScriptVec3i::set_y)
            .register_get_set("z", ScriptVec3i::get_z, ScriptVec3i::set_z);

        engine.register_fn("+", |a: ScriptVec3i, b: ScriptVec3i| -> ScriptVec3i {
            ScriptVec3i::from_vec3i(a.v + b.v)
        });

        engine.register_fn("-", |a: ScriptVec3i, b: ScriptVec3i| -> ScriptVec3i {
            ScriptVec3i::from_vec3i(a.v - b.v)
        });

        engine.register_fn("*", |a: ScriptVec3i, b: ScriptVec3i| -> ScriptVec3i {
            ScriptVec3i::from_vec3i(a.v * b.v)
        });

        engine.register_fn("/", |a: ScriptVec3i, b: ScriptVec3i| -> ScriptVec3i {
            ScriptVec3i::from_vec3i(a.v / b.v)
        });

        engine.register_fn("*", |a: i32, b: ScriptVec3i| -> ScriptVec3i {
            ScriptVec3i::from_vec3i(a * b.v)
        });

        engine.register_fn("*", |a: ScriptVec3i, b: i32| -> ScriptVec3i {
            ScriptVec3i::from_vec3i(a.v * b)
        });

        // Swizzle F3 -> F2
        /*
        engine.register_indexer_get(|o: &mut ScriptVec3f, prop: &str| -> Result<F2, Box<EvalAltResult>> {
            match prop {
                "xz" => {
                    Ok(F2::new(o.x, o.z))
                },
                _ => {
                    Err("F3: Property not found".into())
                }
            }
        });*/
        /*
        // Swizzle F3 -> F3
        engine.register_indexer_get(|o: &mut F3, prop: &str| -> Result<F3, Box<EvalAltResult>> {
            match prop {
                "xxx" => {
                    Ok(F3::new(o.x, o.x, o.x))
                },
                _ => {
                    Err("F3: Property not found".into())
                }
            }
        });*/
    }
}

impl FuncArgs for ScriptVec3i {
    fn parse<C: Extend<rhai::Dynamic>>(self, container: &mut C) {
        container.extend(once(rhai::Dynamic::from(self)));
    }
}

/// ScriptVec3f
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ScriptVec3f {
    pub v                   : Vec3f,
}

impl ScriptVec3f {

    pub fn from(v: ScriptVec3f) -> Self {
        Self {
            v               : vec3f(v.v.x, v.v.y, v.v.z),
        }
    }

    pub fn from_vec3f(v: Vec3f) -> Self {
        Self {
            v,
        }
    }

    pub fn zeros() -> Self {
        Self {
            v               : Vec3f::zero(),
        }
    }

    pub fn new_x(v: f32) -> Self {
        Self {
            v               : splat3f(v),
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            v               : vec3f(x, y, z),
        }
    }

    pub fn get_x(&mut self) -> f32 {
        self.v.x
    }

    pub fn set_x(&mut self, new_val: f32) {
        self.v.x = new_val;
    }

    pub fn get_y(&mut self) -> f32 {
        self.v.y
    }

    pub fn set_y(&mut self, new_val: f32) {
        self.v.y = new_val;
    }

    pub fn get_z(&mut self) -> f32 {
        self.v.z
    }

    pub fn set_z(&mut self, new_val: f32) {
        self.v.z = new_val;
    }

    /// Register to the engine
    pub fn register(engine: &mut Engine) {

        engine.register_type_with_name::<ScriptVec3f>("vec3f")
            .register_fn("vec3f", ScriptVec3f::zeros)
            .register_fn("vec3f", ScriptVec3f::new)
            .register_fn("vec3f", ScriptVec3f::new_x)
            .register_fn("vec3f", ScriptVec3f::from)
            .register_get_set("x", ScriptVec3f::get_x, ScriptVec3f::set_x)
            .register_get_set("y", ScriptVec3f::get_y, ScriptVec3f::set_y)
            .register_get_set("z", ScriptVec3f::get_z, ScriptVec3f::set_z);

        engine.register_fn("+", |a: ScriptVec3f, b: ScriptVec3f| -> ScriptVec3f {
            ScriptVec3f::from_vec3f(a.v + b.v)
        });

        engine.register_fn("-", |a: ScriptVec3f, b: ScriptVec3f| -> ScriptVec3f {
            ScriptVec3f::from_vec3f(a.v - b.v)
        });

        engine.register_fn("*", |a: ScriptVec3f, b: ScriptVec3f| -> ScriptVec3f {
            ScriptVec3f::from_vec3f(a.v * b.v)
        });

        engine.register_fn("/", |a: ScriptVec3f, b: ScriptVec3f| -> ScriptVec3f {
            ScriptVec3f::from_vec3f(a.v / b.v)
        });

        engine.register_fn("*", |a: f32, b: ScriptVec3f| -> ScriptVec3f {
            ScriptVec3f::from_vec3f(a * b.v)
        });

        engine.register_fn("*", |a: ScriptVec3f, b: f32| -> ScriptVec3f {
            ScriptVec3f::from_vec3f(a.v * b)
        });

        // Swizzle F3 -> F2
        /*
        engine.register_indexer_get(|o: &mut ScriptVec3f, prop: &str| -> Result<F2, Box<EvalAltResult>> {
            match prop {
                "xz" => {
                    Ok(F2::new(o.x, o.z))
                },
                _ => {
                    Err("F3: Property not found".into())
                }
            }
        });*/
        /*
        // Swizzle F3 -> F3
        engine.register_indexer_get(|o: &mut F3, prop: &str| -> Result<F3, Box<EvalAltResult>> {
            match prop {
                "xxx" => {
                    Ok(F3::new(o.x, o.x, o.x))
                },
                _ => {
                    Err("F3: Property not found".into())
                }
            }
        });*/

        // Tile

        Tile::register(engine);

        engine.register_fn("get_tile", |loc: ScriptVec3i| -> Tile {
            if let Some(tile) = WORLD.lock().unwrap().get_tile(loc.v) {
                tile.clone()
            } else {
                Tile::new(9)
            }
        });

        engine.register_fn("set_tile", |loc: ScriptVec3i, tile: Tile| {
            WORLD.lock().unwrap().set_tile(loc.v, tile);
            WORLD.lock().unwrap().needs_update = true;
        });

        // Hit Record
        HitRecord::register(engine);

        // Side Enum
        engine.register_type_with_name::<SideEnum>("Side")
            .register_static_module("Side", exported_module!(side_module).into());

    }
}

impl FuncArgs for ScriptVec3f {
    fn parse<C: Extend<rhai::Dynamic>>(self, container: &mut C) {
        container.extend(once(rhai::Dynamic::from(self)));
    }
}

// ScriptValue

#[derive(Debug, Clone)]
pub enum ScriptValueType {
    Color,
}


#[derive(Debug, Clone)]
pub struct ScriptValue {
    pub value_type      : ScriptValueType,

    pub name            : String,

    // Color Index
    pub index           : u8,
}

impl ScriptValue {
    pub fn color(name: String, index: i32) -> Self {
        Self {
            value_type : ScriptValueType::Color,

            name,
            index       : index as u8,
        }
    }

    pub fn register(engine: &mut Engine) {
        engine.register_type_with_name::<ScriptValue>("Value")
            .register_fn("color", ScriptValue::color);
    }
}


// Side Enum

use rhai::plugin::*;
use rhai::Dynamic;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SideEnum {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}

// Create a plugin module with functions constructing the 'Side' variants
#[export_module]
mod side_module
{
    // Constructors
    #[allow(non_upper_case_globals)]
    pub const Top: SideEnum = SideEnum::Top;
    #[allow(non_upper_case_globals)]
    pub const Bottom: SideEnum = SideEnum::Bottom;
    #[allow(non_upper_case_globals)]
    pub const Left: SideEnum = SideEnum::Left;
    #[allow(non_upper_case_globals)]
    pub const Right: SideEnum = SideEnum::Right;
    #[allow(non_upper_case_globals)]
    pub const Front: SideEnum = SideEnum::Front;
    #[allow(non_upper_case_globals)]
    pub const Back: SideEnum = SideEnum::Back;

    // Printing
    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(failure_enum: &mut SideEnum) -> String {
        format!("{failure_enum:?}")
    }

    // '==' and '!=' operators
    #[rhai_fn(global, name = "==", pure)]
    pub fn eq(side_enum: &mut SideEnum, side_enum2: SideEnum) -> bool {
        side_enum == &side_enum2
    }
    #[rhai_fn(global, name = "!=", pure)]
    pub fn neq(side_enum: &mut SideEnum, side_enum2: SideEnum) -> bool {
        side_enum != &side_enum2
    }
}
