use std::fmt::Result;

use sqlx::{Pool, Postgres};


#[derive(Clone, Debug)]
pub struct DBClient{
    pool: Pool<Postgres>,
}

impl DBClient {
    pub async fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
        }
    }
}

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
        api_key: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;
}