use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{
    bson::{doc, Bson}, options::ClientOptions, Client, Collection
};
use crate::models::RpmuHistoryInterval;
use crate::helpers::timer::Timer;
use super::{Database, DatabaseError};
pub struct MongoDB {
    collection: Collection<RpmuHistoryInterval>,
}

#[async_trait]
impl Database for MongoDB {
    async fn init() -> Result<Self, DatabaseError> {
        dotenv::dotenv().ok();
        let mongo_url =
            std::env::var("MONGO_URL").expect("MONGO_URL must be set in .env file");
        let client_options = ClientOptions::parse(&mongo_url)
            .await
            .map_err(DatabaseError::MongoDBError)?;
        let client = Client::with_options(client_options)
            .map_err(DatabaseError::MongoDBError)?;
        let database = client.database("masterdb");
        let collection = database.collection::<RpmuHistoryInterval>("performance_test");
        Ok(MongoDB { collection })
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        self.collection
            .insert_one(data)
            .await
            .map_err(DatabaseError::MongoDBError)?;

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        let result = self.collection.insert_many(data).await;
        match result {
            Ok(_) => {
                let elapsed_time = timer.stop();
                Ok(elapsed_time as u64)
            },
            Err(err) => {
                println!("Error inserting Bulk: {:?}", err);
                Err(DatabaseError::MongoDBError(err))
            }
        }
    }
    
    async fn fetch_all(&self) -> Result<(u64, Vec<RpmuHistoryInterval>), DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        let filter = doc! {};
        let mut cursor = self
            .collection
            .find(filter)
            .await
            .map_err(DatabaseError::MongoDBError)?;
    
        let mut results = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(val) => {
                    results.push(val);
                },
                Err(err) => {
                    println!("Err : {:?}", err);
                    return Err(DatabaseError::UnknownError);
                }
            }
        }

        let elapsed_time = timer.stop();
        Ok((elapsed_time as u64, results))
    }

    async fn fetch_latest_timestamp(&self) -> Result<u64, DatabaseError> {
        let pipeline = vec![
            doc! { "$sort": { "end_time": -1 } },
            doc! { "$limit": 1 },
            doc! { "$project": { "end_time": 1, "_id": 0 } }
        ];
    
        let mut cursor = self.collection
            .aggregate(pipeline)
            .await
            .map_err(DatabaseError::MongoDBError)?;
    
        while let Some(result) = cursor.next().await {
            let doc = result.map_err(DatabaseError::MongoDBError)?;
            
            if let Some(end_time_bson) = doc.get("end_time") {
                return match end_time_bson {
                    Bson::Double(val) => Ok(*val as u64),
                    Bson::Int64(val) => Ok(*val as u64),
                    Bson::Int32(val) => Ok(*val as u64),
                    _ => Err(DatabaseError::UnknownError)
                };
            }
        }
    
        Err(DatabaseError::UnknownError)
    }
}
