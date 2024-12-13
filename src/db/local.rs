use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::RpmuHistoryInterval;
use crate::db::{Database, DatabaseError};
use crate::helpers::timer::Timer;

#[derive(Default)]
pub struct InMemoryDatabase {
    data: Arc<Mutex<HashMap<u64, RpmuHistoryInterval>>>,
}

#[async_trait]
impl Database for InMemoryDatabase {
    async fn init() -> Result<Self, DatabaseError> {
        let db = InMemoryDatabase::default();
        let dummy_record = RpmuHistoryInterval {
            start_time: 1669918678.0,
            count: 1669918678.0,
            end_time: 1234.0,
            units: 4.0,
        };
        db.insert_one(&dummy_record).await?;
        Ok(db)
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        let mut map = self.data.lock().unwrap();
        let id = data.start_time as u64;
        map.insert(id, data.clone());

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        let mut map = self.data.lock().unwrap();
        for item in data.clone() {
            let id = item.start_time as u64;
            map.insert(id, item);
        }

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }
}
