use google_maps::run;

#[tokio::main]
async fn main() -> Result<(), google_maps::error::AppError> {
    run().await
}
