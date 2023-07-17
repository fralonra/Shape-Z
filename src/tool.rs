use crate::prelude::*;
use strum_macros::{EnumIter, Display};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, EnumIter, Display)]
pub enum ToolType {
    WallBuilder,
}

use ToolType::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Tool {
    pub tool_type               : ToolType,

    pub widget_values           : Vec<WidgetValue>
}

impl Tool {
    pub fn new(tool_type: ToolType) -> Self {

        Self {
            tool_type,

            widget_values       : vec![],
        }
    }

    pub fn name(&self) -> String {
        match self.tool_type {
            WallBuilder => "Wall Builder".to_string(),
        }
    }

}