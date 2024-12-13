use async_trait::async_trait;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::{self,engine::any};
use surrealdb::Surreal;
use crate::models::RpmuHistoryInterval;
use super::{Database, DatabaseError};
use crate::helpers::timer::Timer;

pub struct SurrealDB {
    db: Surreal<Any>,
}

#[async_trait]
impl Database for SurrealDB {
    async fn init() -> Result<Self, DatabaseError> {
        dotenv::dotenv().ok();
        let database_url = std::env::var("SURREALDB_URL").expect("SURREALDB_URL must be set in .env file");
        let database = any::connect(&database_url).await.map_err(|e| DatabaseError::SurrealDBError(format!("Connection Error: {:?}", e)))?;

        database.signin(Root {
            username: "sainath",
            password: "sainath",
        }).await.map_err(|e| DatabaseError::SurrealDBError(format!("Signin Error: {:?}", e)))?;
        
        database.use_ns("thorchain").use_db("performance").await.map_err(|e| DatabaseError::SurrealDBError(format!("Namespace/Database Error: {:?}", e)))?;

        Ok(SurrealDB { db : database })
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();
        let _ : Vec<RpmuHistoryInterval> = self.db.insert("rpmu_history").content(data.clone()).await.map_err(|e| DatabaseError::SurrealDBError(format!("Insert Error: {:?}", e)))?;
        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();

        if data.is_empty() {
            return Ok(0);
        }

        let _ : Result<Vec<RpmuHistoryInterval>,DatabaseError> = self.db.insert("rpmu_history").content(data.clone()).await.map_err(|e| DatabaseError::SurrealDBError(format!("Insert Error: {:?}", e)));

        let elapsed_time = timer.stop();
        Ok(elapsed_time as u64)
    }
}