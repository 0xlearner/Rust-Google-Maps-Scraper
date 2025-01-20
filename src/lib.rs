pub mod error;
pub mod models;
pub mod services;
pub mod utils;

use error::AppError;
use models::coordinates::Coordinates;
use models::place::Place;
use models::viewport::Viewport;
use services::google_maps::{build_url, fetch_and_process_data, search_location};
use services::grid_generator::generate_grid;
use utils::json_utils::save_places;
use utils::logger::init_logger;

use dotenvy::dotenv;
use futures::future::join_all;
use std::collections::HashSet;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

pub async fn run() -> Result<(), AppError> {
    init_logger();
    dotenv().ok(); // Load .env file
    let api_key = env::var("GOOGLE_MAPS_API_KEY").expect("API key not found");
    let grid_query = "Karachi";
    let query = "restraunts north nazimabad";
    let start = 20;

    log::info!("Starting search for location: {}", grid_query);
    let result = search_location(&api_key, grid_query).await?;

    log::info!("Extracting viewport from search results"); // Extract the viewport from the first result
    let viewport = Viewport::extract_viewport(&result)?;
    log::info!("Northeast: {:?}", viewport.northeast);
    log::info!("Southwest: {:?}", viewport.southwest);

    log::info!("Generating 5x5 grid"); // Generate a 5x5 grid
    let grid = generate_grid(&viewport, 5, 5);
    crate::debug_log!("5x5 Grid: {:?}", grid);

    log::info!("Initializing HTTP client and shared state");
    let client = reqwest::Client::new();

    // Create a shared HashSet to track unique place_ids
    let unique_places: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    // Create a shared vector to store all unique places
    let all_places: Arc<Mutex<Vec<Place>>> = Arc::new(Mutex::new(Vec::new()));

    // Flatten the grid into a single vector of (lat, long) tuples
    let grid_points: Vec<(f64, f64)> = grid.into_iter().flatten().collect();

    log::info!("Processing grid points in parallel"); // Process grid points in parallel with a rate limit of 5 requests per second
    for chunk in grid_points.chunks(5) {
        let mut chunk_tasks = Vec::new();
        for &(lat, long) in chunk {
            let url = build_url(lat, long, start, query);
            let client = client.clone();
            let unique_places = Arc::clone(&unique_places);
            let all_places = Arc::clone(&all_places);
            let searched_coords = Coordinates {
                lat: Some(lat),
                long: Some(long),
            };
            let task = tokio::spawn(async move {
                match fetch_and_process_data(
                    &client,
                    &url,
                    &unique_places,
                    &all_places,
                    searched_coords,
                )
                .await
                {
                    Ok(_) => log::info!(
                        "Successfully processed data for lat: {}, long: {}",
                        lat,
                        long
                    ),
                    Err(e) => crate::error_log!(
                        "Error processing data for lat: {}, long: {}: {}",
                        lat,
                        long,
                        e
                    ),
                }
            });
            chunk_tasks.push(task);
        }

        log::info!("Waiting for tasks in the current chunk to complete"); // Wait for all tasks in the current chunk to complete
        join_all(chunk_tasks).await;

        log::info!("Sleeping for 1 second to enforce rate limit"); // Sleep for 1 second to enforce the rate limit
        sleep(Duration::from_secs(1)).await;
    }

    log::info!("Saving all unique places to JSON file"); // Save all unique places to a single JSON file
    let all_places = all_places.lock().unwrap();
    save_places(&*all_places)?;

    log::info!(
        "Exported {} unique places to all_places_output.json",
        all_places.len()
    );

    Ok(())
}
