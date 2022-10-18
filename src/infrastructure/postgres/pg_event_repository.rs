use sqlx::{Pool, Postgres};

use crate::common::{EventPublisherClient, EventPublishError, RawEvent};

#[derive(Clone)]
pub struct PgEventRepository {
    pool: Pool<Postgres>,
}

impl PgEventRepository {
    pub fn build(pool: Pool<Postgres>) -> PgEventRepository {
        PgEventRepository { pool }
    }
}

#[tonic::async_trait]
impl EventPublisherClient for PgEventRepository {
    async fn publish(&self, event: RawEvent) -> Result<(), EventPublishError> {
        let sql = r#"
               insert into events (kind, data)
               values ($1, $2::json)"#;

        sqlx::query(sql)
            .bind(event.0)
            .bind(event.1)
            .execute(&self.pool)
            .await
            .map_err(|_| EventPublishError::PersistentError)?;

        Ok(())
    }
}