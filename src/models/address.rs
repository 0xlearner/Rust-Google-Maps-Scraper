use crate::utils::json_utils::{extract_value, get_nested_value};

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Address {
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub state: Option<String>,
    pub country_code: Option<String>,
}

// Builds an `Address` from JSON data.
pub fn build_address(place: &Value) -> Address {
    let lookup = |indexes: &[usize]| get_nested_value(place, indexes);

    Address {
        street_address: extract_value(lookup(&[18])),
        city: extract_value(lookup(&[183, 1, 3])),
        zip: extract_value(lookup(&[183, 1, 4])),
        state: extract_value(lookup(&[183, 1, 5])),
        country_code: extract_value(lookup(&[183, 1, 6])),
    }
}
