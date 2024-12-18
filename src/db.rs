pub mod local;
pub mod mongo_db;
pub mod postgres;
pub mod surreal_db;
pub mod rocks_db;
pub mod level_db;

use async_trait::async_trait;
use thiserror::Error;
use crate::models::RpmuHistoryInterval;
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("MongoDB error: {0}")]
    MongoDBError(#[from] mongodb::error::Error),
    
    #[error("PostgreSQL error: {0}")]
    PostgresError(#[from] sqlx::Error),

    #[error("SurrealDB error : {0}")]
    SurrealDBError(String),

    #[error("LevelDB error : {0}")]
    LevelDBError(String),

    #[error("RocksDB error : {0}")]
    RocksDBError(String),
}

#[async_trait]
pub trait Database {
    async fn init() -> Result<Self, DatabaseError>
    where
        Self: Sized;
    
    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError>;
    
    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError>;
}