use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, QueryBuilder};
use crate::models::RpmuHistoryInterval;
use super::{Database, DatabaseError};
use crate::helpers::timer::Timer;

pub struct PostgresDB {
    pool: Pool<Postgres>,
}

#[async_trait]
impl Database for PostgresDB {
    async fn init() -> Result<Self, DatabaseError> {
        dotenv::dotenv().ok();
        let database_url =
            std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set in .env file");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(DatabaseError::PostgresError)?;
        Ok(PostgresDB { pool })
    }

    async fn insert_one(&self, data: &RpmuHistoryInterval) -> Result<u64, DatabaseError> {
        let query = "INSERT INTO rpmu_history (count, end_time, start_time, units) VALUES ($1, $2, $3, $4)";
        sqlx::query(query)
            .bind(data.count)
            .bind(data.end_time)
            .bind(data.start_time)
            .bind(data.units)
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::PostgresError)?;
        Ok(1)
    }

    async fn insert_many(&self, data: Vec<RpmuHistoryInterval>) -> Result<u64, DatabaseError> {
        let mut timer = Timer::init();
        timer.start();
        
        if data.is_empty() {
            return Ok(0);
        }

        // Create a query builder for bulk insert
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO rpmu_history (count, end_time, start_time, units) "
        );

        // Prepare the values clause
        query_builder.push_values(&data, |mut b, item| {
            b.push_bind(item.count)
                .push_bind(item.end_time)
                .push_bind(item.start_time)
                .push_bind(item.units);
        });

        let _ = query_builder
            .build()
            .execute(&self.pool)
            .await
            .map_err(DatabaseError::PostgresError)?
            .rows_affected();

        Ok(timer.stop() as u64)
    }


    async fn fetch_all(&self) -> Result<(u64,Vec<RpmuHistoryInterval>), DatabaseError> {
        let mut timer = Timer::init();
        timer.start();
        let query = "SELECT count, end_time, start_time, units FROM rpmu_history";
        let records = sqlx::query_as::<_, RpmuHistoryInterval>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(DatabaseError::PostgresError)?;
        let elapsed_time = timer.stop();
        Ok((elapsed_time as u64,records))
    }

    async fn fetch_latest_timestamp(&self) -> Result<u64, DatabaseError> {
        let query = "SELECT MAX(start_time) AS latest_start_time FROM rpmu_history";
        let row: (Option<f64>,) = sqlx::query_as(query)
            .fetch_one(&self.pool)
            .await
            .map_err(DatabaseError::PostgresError)?;

        match row.0 {
            Some(latest_start_time) => {
                Ok(latest_start_time as u64)
            },
            None => Err(DatabaseError::UnknownError),
        }
    }
}
