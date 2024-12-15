use async_trait::async_trait;
use std::sync::Arc;
use crate::models::RpmuHistoryInterval; 
use crate::helpers::timer::Timer;
use crate::db::Database as DatabaseTrait; 
use crate::db::DatabaseError;

use rusty_leveldb::{AsyncDB, Options};


pub struct LevelDB {
    db: Arc<AsyncDB>,
}

#[async_trait]
impl DatabaseTrait for LevelDB {
    async fn init() -> Result<Self, DatabaseError> {
        let db = AsyncDB::new("leveldb/thorchian", Options::default()).unwrap();
        Ok(LevelDB { db: Arc::new(db) })
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        let unique_id = data.start_time.to_string();
        self.db.put(unique_id.as_bytes().to_owned(), serde_json::to_vec(data).map_err(|err| DatabaseError::LevelDBError(format!("Serialization Error : {}", err)))?)
            .await
            .map_err(|err| DatabaseError::LevelDBError(format!("Insertion Error : {}", err)))?;
        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        for item in data {
            let unique_id = item.start_time.to_string();
            self.db.put(unique_id.as_bytes().to_owned(), serde_json::to_vec(&item).map_err(|err| DatabaseError::LevelDBError(format!("Serialization Error : {}", err)))?)
                .await
                .map_err(|err| DatabaseError::LevelDBError(format!("Insertion Error : {}", err)))?;
        }

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }
}
