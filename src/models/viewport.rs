use crate::error::AppError;
use serde_json::Value;

#[derive(Debug)]
pub struct Viewport {
    pub northeast: (f64, f64),
    pub southwest: (f64, f64),
}

impl Viewport {
    pub fn extract_viewport(json: &Value) -> Result<Self, AppError> {
        if let Some(result) = json.get("results").and_then(|v| v.get(0)) {
            if let Some(geometry) = result.get("geometry") {
                if let Some(viewport) = geometry.get("viewport") {
                    let northeast_lat = viewport["northeast"]["lat"]
                        .as_f64()
                        .ok_or(AppError::ValueExtractionFailed)?;
                    let northeast_lng = viewport["northeast"]["lng"]
                        .as_f64()
                        .ok_or(AppError::ValueExtractionFailed)?;
                    let southwest_lat = viewport["southwest"]["lat"]
                        .as_f64()
                        .ok_or(AppError::ValueExtractionFailed)?;
                    let southwest_lng = viewport["southwest"]["lng"]
                        .as_f64()
                        .ok_or(AppError::ValueExtractionFailed)?;

                    return Ok(Viewport {
                        northeast: (northeast_lat, northeast_lng),
                        southwest: (southwest_lat, southwest_lng),
                    });
                }
            }
        }
        Err(AppError::NoViewport)
    }
}
