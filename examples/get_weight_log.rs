use chrono::{Duration, Utc};
use dotenv::dotenv;
use fitbit_rs::Client;

#[tokio::main]
async fn main() -> fitbit_rs::Result<()> {
    dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID env variable is required");
    let client_secret =
        std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET env variable is required");
    let client = Client::new(&client_id, &client_secret)
        .with_cache("tokens")
        .auth_interactive()
        .await;
    let start_date = Utc::now().naive_utc().date() - Duration::days(31);
    let log = client
        .body_time_series()
        .get_weight_time_series_by_date_range(Some(start_date), None, None)
        .await?;
    dbg!(log);
    Ok(())
}
