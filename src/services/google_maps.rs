use crate::{debug_log, error_log};
use crate::{
    error::AppError,
    models::{Coordinates, Place},
    utils::http_client::create_headers,
    utils::json_utils::{build_results, prepare},
};

use serde_json::Value;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub async fn search_location(api_key: &str, query: &str) -> Result<Value, AppError> {
    let url = format!(
        "https://maps.googleapis.com/maps/api/place/textsearch/json?query={}&key={}",
        query, api_key
    );

    log::info!("Fetching search location");
    let response = reqwest::get(&url).await?;

    log::info!("Search API Response status: {}", response.status());
    let json: Value = response.json().await?;

    Ok(json)
}

pub fn build_url(lat: f64, long: f64, start: u32, query: &str) -> String {
    let base_url = "https://www.google.com/search";
    let params = format!(
        "tbm=map&authuser=0&hl=en&pb=!4m12!1m3!1d11704.398661666706!2d{}!3d{}!2m3!1f0!2f0!3f0!3m2!1i445!2i621!4f13.1!7i{}!10b1!12m21!1m2!18b1!30b1!2m3!5m1!6e2!20e3!10b1!12b1!13b1!16b1!17m1!3e1!20m3!5e2!6b1!14b1!46m1!1b0!94b1!96b1!19m4!2m3!1i360!2i120!4i8!20m57!2m2!1i203!2i100!3m2!2i4!5b1!6m6!1m2!1i86!2i86!1m2!1i408!2i240!7m42!1m3!1e1!2b0!3e3!1m3!1e2!2b1!3e2!1m3!1e2!2b0!3e3!1m3!1e8!2b0!3e3!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e9!2b1!3e2!1m3!1e10!2b0!3e3!1m3!1e10!2b1!3e2!1m3!1e10!2b0!3e4!2b1!4b1!9b0!22m5!1s1pk8Z7ObMvuRkdUPhKD9iAY%3A60!2s1i%3A0%2Ct%3A150715%2Cp%3A1pk8Z7ObMvuRkdUPhKD9iAY%3A60!7e81!12e3!17s1pk8Z7ObMvuRkdUPhKD9iAY%3A66!24m105!1m32!13m9!2b1!3b1!4b1!6i1!8b1!9b1!14b1!20b1!25b1!18m21!3b1!4b1!5b1!6b1!9b1!12b1!13b1!14b1!17b1!20b1!21b1!22b1!25b1!27m1!1b0!28b0!32b0!33m1!1b1!34b0!36e1!10m1!8e3!11m1!3e1!14m1!3b1!17b1!20m2!1e3!1e6!24b1!25b1!26b1!29b1!30m1!2b1!36b1!39m3!2m2!2i1!3i1!43b1!52b1!54m1!1b1!55b1!56m1!1b1!65m5!3m4!1m3!1m2!1i224!2i298!71b1!72m19!1m5!1b1!2b1!3b1!5b1!7b1!4b1!8m10!1m6!4m1!1e1!4m1!1e3!4m1!1e4!3sother_user_reviews!6m1!1e1!9b1!89b1!98m3!1b1!2b1!3b1!103b1!113b1!114m3!1b1!2m1!1b1!117b1!122m1!1b1!125b0!126b1!127b1!26m4!2m3!1i80!2i92!4i8!30m0!34m18!2b1!3b1!4b1!6b1!8m6!1b1!3b1!4b1!5b1!6b1!7b1!9b1!12b1!14b1!20b1!23b1!25b1!26b1!37m1!1e81!42b1!47m0!49m9!3b1!6m2!1b1!2b1!7m2!1e3!2b1!8b1!9b1!50m4!2e2!3m2!1b1!3b1!67m2!7b1!10b1!69i713&q={}&nfpr=1&tch=1&ech=1&psi=1pk8Z7ObMvuRkdUPhKD9iAY.1732024792533.1",

        lat, long, start, query
    );
    format!("{}?{}", base_url, params)
}

pub async fn fetch_and_process_data(
    client: &reqwest::Client,
    url: &str,
    unique_places: &Arc<Mutex<HashSet<String>>>,
    all_places: &Arc<Mutex<Vec<Place>>>,
    searched_coords: Coordinates,
) -> Result<(), AppError> {
    debug_log!("Fetching data from URL: {}", url); // Make the request and process the response

    let response = client.get(url).headers(create_headers()).send().await?;

    debug_log!("Response status: {}, URL: {}", response.status(), url);

    if !response.status().is_success() {
        error_log!(
            "Request returned non-200 status code: {} for URL: {}",
            response.status(),
            url
        );
        return Ok(()); // Return early but successfully to continue with other requests
    }

    let raw_input = response.text().await?;

    debug_log!("Preparing and processing JSON data");
    let prepared_data = prepare(&raw_input)?;
    let list_results = build_results(&prepared_data, searched_coords.clone())?;

    debug_log!("Filtering out duplicate places"); // Filter out duplicate places
    for place in list_results {
        let place_id = place.place_id.clone();
        // Acquire a lock on the shared HashSet
        let mut unique_places = unique_places.lock().unwrap();
        if let Some(place_id) = place_id {
            // If the place_id is not in the HashSet, add it and keep the place
            if !unique_places.contains(&place_id) {
                unique_places.insert(place_id);
                let mut all_places = all_places.lock().unwrap();
                all_places.push(place);
            }
        } else {
            log::warn!("Place with no place_id found, including it in results"); // If the place has no place_id, include it (but this is unlikely)
            let mut all_places = all_places.lock().unwrap();
            all_places.push(place);
        }
    }

    Ok(())
}
