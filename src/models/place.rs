use super::{Address, Coordinates};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Place {
    pub address: Address,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub place_id: Option<String>,
    pub phone: Option<String>,
    pub reviews_count: Option<u64>,
    pub avg_reviews: Option<f64>,
    pub reviews_url: Option<String>,
    pub price_range: Option<String>,
    pub website: Option<String>,
    pub online_delivery: Option<String>,
    pub online_delivery_link: Option<String>,
    pub coordinates: Coordinates,
    pub searched_coords: Coordinates,
}
