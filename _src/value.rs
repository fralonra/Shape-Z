use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    // Name, Value, Min, Max
    Int(String, i32, i32, i32),
    Float(String, f32, f32, f32),
    Sdf2D(SDF2D),
    Sdf3D(SDF3D),
}

pub use Value::*;

impl Value {
    /*
    pub fn create_widget(&self) -> Box<dyn Widget> {
        match self {
            Int(name, value, min, max) => {
                let mut w = Box::new(IntSlider::new());
                w.set_text(name.clone());
                w.set_range(*min, *max);
                w.set_value(*value);
                w
            }
        }
    }*/

    pub fn get_int(&self) -> i32 {
        match self {
            Int(_name, value, _min, _max) => {
                *value
            },
            _ => {
                0
            }
        }
    }

        pub fn get_float(&self) -> f32 {
        match self {
            Float(_name, value, _min, _max) => {
                *value
            },
            _ => {
                0.0
            }
        }
    }
}