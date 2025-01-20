# Rust Google Maps Scraper

This is a Rust-based project that interacts with the Google Maps API to search for locations, extract place details, and save the results to a JSON file. It was developed as part of my journey to learn Rust by building practical applications.

## Features

- **Google Maps API Integration**: Fetches location data using the Google Maps Places API.
- **Grid-Based Search**: Divides a geographic area into a grid and searches for places within each grid cell.
- **Data Processing**: Extracts and processes place details such as name, address, coordinates, reviews, and more.
- **Concurrency**: Uses `tokio` for asynchronous HTTP requests and parallel processing.
- **Shared State with `Arc` and `Mutex`**: Safely shares data across threads using `Arc` (Atomic Reference Counting) and `Mutex` (Mutual Exclusion).
- **Duplicate Filtering with `HashSet`**: Uses `HashSet` to filter out duplicate places based on their `place_id`.
- **Logging**: Implements structured logging using `log4rs` for debugging and monitoring.
- **Modular Design**: The code is organized into modules (`models`, `services`, `utils`) for better maintainability.

## Getting Started

### Prerequisites

- Rust installed (via [rustup](https://rustup.rs/)).
- A Google Maps API key (get one from the [Google Cloud Console](https://console.cloud.google.com/)).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/rust-google-maps-scraper.git
   cd rust-google-maps-scraper
2. Add your Google Maps API key to the main.rs file:
    ```bash
    let api_key = "YOUR_API_KEY_HERE";
3. Build and run the project:
    ```bash
    cargo run
###  Configuration
- **Logging**: Configure logging in `log4rs.yaml`. By default, logs are saved to `debug.log` and `error.log`.
- **Search Query**: Modify the `query` variable in `main.rs` to search for different types of `places`.

- **Grid Size**: Adjust the grid size (rows and columns) in the `generate_grid` function.
### Project Structure
```
src/
├── main.rs                # Entry point of the application
├── lib.rs                 # Main module exports
├── models/                # Data structures (e.g., Place, Address, Coordinates)
├── services/              # Business logic (e.g., Google Maps API interactions)
├── utils/                 # Utility functions (e.g., JSON processing, logging)
├── error.rs               # Custom error handling
