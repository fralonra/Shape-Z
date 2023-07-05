use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Tool {
    pub widget_values           : Vec<WidgetValue>
}

impl Tool {
    pub fn new(script: String) -> Self {

        Self {
            widget_values       : vec![],
        }
    }

}