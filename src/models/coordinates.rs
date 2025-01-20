use crate::utils::json_utils::{extract_value, get_nested_value};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Coordinates {
    pub lat: Option<f64>,
    pub long: Option<f64>,
}

// Builds `Coordinates` from JSON data.
pub fn build_coordinates(place: &Value) -> Coordinates {
    let lookup = |indexes: &[usize]| get_nested_value(place, indexes);

    Coordinates {
        long: extract_value(lookup(&[9, 2])),
        lat: extract_value(lookup(&[9, 3])),
    }
}
