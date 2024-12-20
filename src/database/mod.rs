//! Provides access to the database.
use chrono::NaiveDateTime;
pub use client::PgDbClient;
use sqlx::types::BigDecimal;
use std::fmt::{Display, Formatter};

mod client;
pub(crate) mod operations;

/// Snowflake ID.
type Snowflake = String;

/// A unique identifier for a workflow run.
#[derive(Clone, Copy, Debug)]
pub struct RunId(pub u64);

/// Postgres doesn't support unsigned integers.
impl sqlx::Type<sqlx::Postgres> for RunId {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl From<i64> for RunId {
    fn from(value: i64) -> RunId {
        RunId(value as u64)
    }
}

impl Display for RunId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Represents a user.
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "users")]
pub struct UserModel {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub access_token: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_bets")]
pub struct UserBetsModel {
    pub user_id: Snowflake,
    pub bet_id: Snowflake,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "bet")]
pub struct BetModel {
    pub id: Snowflake,
    pub casino: String,
    pub amount: BigDecimal,
    pub created_at: NaiveDateTime,
}
