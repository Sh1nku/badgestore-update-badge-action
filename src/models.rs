use crate::helpers::get_input;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BadgeInput {
    left_label: Option<String>,
    left_color: Option<String>,
    right_label: Option<String>,
    right_color: Option<String>,
}

pub fn parse_inputs() -> BadgeInput {
    BadgeInput {
        left_label: get_input("left-label"),
        left_color: get_input("left-color"),
        right_label: get_input("right-label"),
        right_color: get_input("right-color"),
    }
}
