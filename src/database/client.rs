use super::operations::{create_bet, create_user, get_user};
use crate::database::UserModel;
use anyhow::Context;
use sqlx::{types::BigDecimal, PgPool};
use std::str::FromStr;

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

    pub async fn get_user(&self, user_id: &str) -> anyhow::Result<UserModel> {
        let user = get_user(&self.pool, user_id)
            .await?
            .context("User not found")?;

        Ok(user)
    }

    pub async fn create_bet(
        &self,
        amount: &str,
        casino: &str,
        user_id: &str,
    ) -> anyhow::Result<String> {
        create_bet(
            &self.pool,
            casino,
            BigDecimal::from_str(amount).context("Failed to parse amount")?,
            user_id,
        )
        .await
    }
}
