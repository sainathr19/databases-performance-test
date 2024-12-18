use crate::{db::{level_db::LevelDB, local::InMemoryDatabase, mongo_db::MongoDB, postgres::PostgresDB, rocks_db::RocksDBWrapper, surreal_db::SurrealDB, Database}, models::RpmuHistoryResponse};
use reqwest;
use tokio::time;

pub async fn fetch_latest_data() -> Result<(), Box<dyn std::error::Error>> {
    let db = InMemoryDatabase::init().await?;
    let mongo = MongoDB::init().await?;
    let postgres = PostgresDB::init().await?;
    let surrealdb = SurrealDB::init().await?;
    let rocksdb = RocksDBWrapper::init().await?;
    let level = LevelDB::init().await?;

    loop {
        let count = 400 ;
        let url: String = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count={}",
            count
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
        match db.insert_many(resp.intervals.clone()).await {
            Ok(time_taken) => {
                println!("InMemoryDatabase : {:.2} ms", time_taken);
            },
            Err(err) => {
                println!("Error inserting into InMemoryDatabase: {:?}", err);
                break;
            }
        }

        match mongo.insert_many(resp.intervals.clone()).await {
            Ok(time_taken) => {
                println!("MongoDB : {:.2} ms", time_taken);
            },
            Err(err) => {
                println!("Error inserting into MongoDB: {:?}", err);
                break;
            }
        }

        match postgres.insert_many(resp.intervals.clone()).await {
            Ok(time_taken) => {
                println!("PostGres : {:.2} ms", time_taken);
            },
            Err(err) => {
                println!("Error inserting into PostGres: {:?}", err);
                break;
            }
        }

        match surrealdb.insert_many(resp.intervals.clone()).await {
            Ok(time_taken) => {
                println!("SurrelDB : {:.2} ms", time_taken);
            },
            Err(err) => {
                println!("Error inserting into SurrealDB: {:?}", err);
                break;
            }
        }

        match rocksdb.insert_many(resp.intervals.clone()).await {
            Ok(time_taken) => {
                println!("RocksDB : {:.2} ms.", time_taken);
            },
            Err(err) => {
                println!("Error inserting into RocksDB: {:?}", err);
                break;
            }
        }

        match level.insert_many(resp.intervals.clone()).await {
            Ok(time_taken) => {
                println!("Inserted into LevelDB in {:.2} ms.", time_taken);
            },
            Err(err) => {
                println!("Error inserting into LevelDB: {:?}", err);
                break;
            }
        }

        println!("Waiting for 3 seconds before the next request...");
        time::sleep(time::Duration::from_secs(3)).await;
    }

    Ok(())
}
