use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    // Name, Value, Min, Max
    Int(String, i32, i32, i32)
}

impl Value {
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
    }

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
}