use crate::debug_log;
use crate::error::AppError;
use crate::models::address::build_address;
use crate::models::coordinates::build_coordinates;
use crate::models::coordinates::Coordinates;
use crate::models::place::Place;

use serde_json::{to_string_pretty, Value};
use std::fs;
use std::io::Write;

pub fn save_json_to_file(json: &Value, filename: &str) -> Result<(), AppError> {
    let formatted_json = to_string_pretty(json)?;
    let mut file = fs::File::create(filename)?;
    file.write_all(formatted_json.as_bytes())?;
    debug_log!("JSON response saved to {}", filename);
    Ok(())
}

pub fn save_places(places: &[Place]) -> Result<(), AppError> {
    let json_output = to_string_pretty(&*places)?;
    std::fs::write("all_places_output.json", json_output)?;
    Ok(())
}

// Extracts a nested value from a JSON structure using a list of indexes.
pub fn get_nested_value<'a>(data: &'a Value, indexes: &[usize]) -> Option<&'a Value> {
    let mut current = data;
    for &index in indexes {
        current = current.get(index)?;
    }
    Some(current)
}

// Prepares raw input data by cleaning and parsing it into JSON.
pub fn prepare(input: &str) -> Result<Value, AppError> {
    let prepared = input.replace("/*\"\"*/", "");
    let json: Value = serde_json::from_str(&prepared)?;

    save_json_to_file(&json, "raw_response.json")?;

    if let Some(d_str) = json.get("d").and_then(|v| v.as_str()) {
        let cleaned_d = d_str.trim_start_matches(")]}'\n");
        let d_json: Value = serde_json::from_str(cleaned_d)?;

        let mut modified_json = json.clone();
        modified_json["d"] = d_json;

        save_json_to_file(&modified_json, "cleaned_response.json")?;

        if let Some(first_key_value) = modified_json
            .get("d")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get(1))
            .and_then(|v| v.as_array())
        {
            Ok(serde_json::Value::Array(
                first_key_value
                    .iter()
                    .filter_map(|item| item.get(14).cloned())
                    .collect(),
            ))
        } else {
            Err(AppError::ArrayExtractionFailed)
        }
    } else {
        Err(AppError::InvalidJson(
            "Missing 'd' field or invalid format".into(),
        ))
    }
}

// Builds a list of `Place` results from JSON data.
pub fn build_results(json: &Value, searched_coords: Coordinates) -> Result<Vec<Place>, AppError> {
    if let Some(arr) = json.as_array() {
        let results: Vec<Place> = arr
            .iter()
            .filter_map(|place| {
                let lookup = |indexes: &[usize]| get_nested_value(place, indexes);

                Some(Place {
                    address: build_address(place),
                    name: extract_value(lookup(&[11])),
                    tags: extract_value(lookup(&[13])),
                    notes: extract_value(lookup(&[25, 15, 0, 2])),
                    place_id: extract_value(lookup(&[78])),
                    phone: extract_value(lookup(&[178, 0, 0])),
                    reviews_count: extract_value(lookup(&[4, 8])),
                    avg_reviews: extract_value(lookup(&[4, 7])),
                    reviews_url: extract_value(lookup(&[4, 3, 0])),
                    price_range: extract_value(lookup(&[4, 10])),
                    website: extract_value(lookup(&[7, 0])),
                    online_delivery: extract_value(lookup(&[75, 0, 0, 2, 0, 0, 2, 1])),
                    online_delivery_link: extract_value(lookup(&[75, 0, 0, 2, 0, 1, 2, 0])),
                    coordinates: build_coordinates(place),
                    searched_coords: searched_coords.clone(),
                })
            })
            .collect();

        Ok(results)
    } else {
        Err(AppError::NotAnArray)
    }
}

// Defines a trait for types that can be extracted from JSON.
pub trait Extractable: Sized {
    fn extract(value: &Value) -> Option<Self>;
}

// Implement Extractable for f64
impl Extractable for f64 {
    fn extract(value: &Value) -> Option<Self> {
        value.as_f64()
    }
}

// Implement Extractable for u64
impl Extractable for u64 {
    fn extract(value: &Value) -> Option<Self> {
        value.as_u64()
    }
}

// Implement Extractable for String
impl Extractable for String {
    fn extract(value: &Value) -> Option<Self> {
        value.as_str().map(String::from)
    }
}

// Implement Extractable for Vec<String>
impl Extractable for Vec<String> {
    fn extract(value: &Value) -> Option<Self> {
        value.as_array().map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_str().map(String::from))
                .collect()
        })
    }
}

// Extracts a value of type T from a JSON value.
pub fn extract_value<T: Extractable>(value: Option<&Value>) -> Option<T> {
    value.and_then(|v| T::extract(v))
}
