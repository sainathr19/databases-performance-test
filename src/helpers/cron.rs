use crate::{db::{local::InMemoryDatabase, Database}, models::{RpmuHistoryResponse}};
use reqwest;
use tokio::time;
use crate::helpers::timer::Timer;

pub async fn fetch_latest_data() -> Result<(), Box<dyn std::error::Error>> {
    let db = InMemoryDatabase::init().await?;

    loop {
        let from = db.fetch_latest_timestamp().await?;
        let count = 20;
        let url: String = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count={}&from={}",
            count, from
        );
        println!("Fetching URL: {}", &url);
        
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            println!("Request failed with status: {}", response.status());
            break;
        }

        let resp: RpmuHistoryResponse = response.json().await?;

        if resp.intervals.is_empty() {
            println!("No intervals found, exiting loop.");
            break;
        }

        let mut timer = Timer::init();
        timer.start();
        time::sleep(time::Duration::from_secs(3)).await;
        match db.insert_many(resp.intervals.clone()).await {
            Ok(val) => {
                let elapsed_time = timer.stop();
                println!("Inserted {} intervals in {:.2} seconds.", val, elapsed_time);
            },
            Err(err) => {
                println!("Error inserting: {:?}", err);
                break;
            }
        }

        println!("Waiting for 3 seconds before the next request...");
        time::sleep(time::Duration::from_secs(3)).await;
    }

    Ok(())
}
