use super::operations::{create_bet, create_user};
use crate::database::{BetModel, UserModel};
use sqlx::PgPool;
// use super::RunId;

/// Provides access to a database using sqlx operations.
#[derive(Clone)]
pub struct PgDbClient {
    pool: PgPool,
}

impl PgDbClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: UserModel) -> anyhow::Result<()> {
        create_user(&self.pool, &user).await
    }

    pub async fn create_bet(&self, profit: BetModel) -> anyhow::Result<()> {
        create_bet(&self.pool, &profit).await
    }
}
