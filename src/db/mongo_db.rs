use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client, Collection};
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

}
