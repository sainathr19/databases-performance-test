use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::RpmuHistoryInterval;
use crate::db::{Database, DatabaseError};

#[derive(Default)]
pub struct InMemoryDatabase {
    data: Arc<Mutex<HashMap<u64, RpmuHistoryInterval>>>,
}

#[async_trait]
impl Database for InMemoryDatabase {
    async fn init() -> Result<Self, DatabaseError> {
        let db = InMemoryDatabase::default();
        let dummy_record = RpmuHistoryInterval {
            start_time: 1733077078 as f64,
            count : 1733077078 as f64,
            end_time : 1234 as f64,
            units : 4 as f64
        };
        db.insert_one(&dummy_record).await?;
        Ok(db)
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut map = self.data.lock().unwrap();
        let id = data.start_time as u64;
        map.insert(id, data.clone());
        Ok(id)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut map = self.data.lock().unwrap();
        for item in data.clone() {
            let id = item.start_time as u64;
            map.insert(id, item);
        }
        Ok(data.len() as u64)
    }

    async fn fetch_one(&self, filter: Option<serde_json::Value>) -> Result<RpmuHistoryInterval, DatabaseError> {
        let map = self.data.lock().unwrap();
        if let Some(filter_value) = filter {
            let start_time = filter_value.as_u64().ok_or(DatabaseError::UnknownError)?;
            if let Some(interval) = map.get(&start_time) {
                return Ok(interval.clone());
            }
        }
        Err(DatabaseError::UnknownError)
    }

    async fn fetch_all(&self, _filter: Option<serde_json::Value>) -> Result<Vec<RpmuHistoryInterval>, DatabaseError> {
        let map = self.data.lock().unwrap();
        let intervals: Vec<RpmuHistoryInterval> = map.values().cloned().collect();
        Ok(intervals)
    }

    async fn fetch_latest_timestamp(&self) -> Result<u64, DatabaseError> {
        let map = self.data.lock().unwrap();
        if let Some((latest_start_time, _)) = map.iter().max_by_key(|(&end_time, _)| end_time) {
            return Ok(*latest_start_time);
        }
        Err(DatabaseError::UnknownError)
    }
}
