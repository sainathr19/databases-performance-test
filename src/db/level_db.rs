use async_trait::async_trait;
use leveldb::database::Database;
use leveldb::options::Options;
use leveldb::iterator::Iterable;
use std::sync::Arc;
use crate::models::RpmuHistoryInterval; // Assuming you have a similar model
use crate::helpers::timer::Timer; // Assuming you have a Timer helper
use super::{Database, DatabaseError};

pub struct LevelDB {
    db: Arc<Database>,
}

#[async_trait]
impl Database for LevelDB {
    async fn init() -> Result<Self, DatabaseError> {
        let mut options = Options::new();
        options.create_if_missing = true; // Create the database if it doesn't exist
        let db = Database::open("path/to/your/db", options)
            .map_err(DatabaseError::LevelDBError)?; // Handle LevelDB specific errors
        Ok(LevelDB { db: Arc::new(db) })
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        self.db.put(data.key.as_bytes(), data.value.as_bytes())
            .map_err(DatabaseError::LevelDBError)?; // Handle LevelDB specific errors

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        for item in data {
            self.db.put(item.key.as_bytes(), item.value.as_bytes())
                .map_err(DatabaseError::LevelDBError)?; // Handle LevelDB specific errors
        }

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }
}
