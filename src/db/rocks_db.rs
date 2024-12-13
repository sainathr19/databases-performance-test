use async_trait::async_trait;
use crate::models::RpmuHistoryInterval;
use crate::helpers::timer::Timer;
use super::{Database, DatabaseError};
use rocksdb::DB;
use std::path::Path;

pub struct RocksDBWrapper {
    db: DB,
}

#[async_trait]
impl Database for RocksDBWrapper {
    async fn init() -> Result<Self, DatabaseError> {
        let path = "rocksdb/thorchain";
        let db = DB::open_default(Path::new(path)).map_err(|e| DatabaseError::RocksDBError(format!("Initialization Error: {}", e)))?;
        Ok(RocksDBWrapper { db })
    }
    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        let serialized_data = bincode::serialize(data).map_err(|e| DatabaseError::RocksDBError(format!("Serialization Error: {}", e)))?;
        let key = bincode::serialize(&data.start_time).map_err(|e| DatabaseError::RocksDBError(format!("Serialization Error: {}", e)))?;

        let res = self.db.put(&key, &serialized_data);
        
        if res.is_err() {
            return Err(DatabaseError::RocksDBError(format!("Insertion Failed: {}", res.unwrap_err())))
        }
        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();
        for interval in data{
            let serialized_interval = bincode::serialize(&interval).map_err(|e| DatabaseError::RocksDBError(format!("Serialization Error: {}", e)))?;
            let key = bincode::serialize(&interval.start_time).map_err(|e| DatabaseError::RocksDBError(format!("Serialization Error: {}", e)))?;

            self.db.put(&key, &serialized_interval).map_err(|e| DatabaseError::RocksDBError(format!("Insertion Failed: {}", e)))?;
        }
        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }
}
