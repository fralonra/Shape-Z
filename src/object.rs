use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Object {
    pub tools                   : Vec<Tool>
}

impl Object {
    pub fn new() -> Self {

        Self {
            tools               : vec![],
        }
    }

}