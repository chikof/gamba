//! Provides access to the database.
use chrono::{NaiveDate, NaiveDateTime};
pub use client::PgDbClient;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use std::fmt::{Display, Formatter};

mod client;
pub(crate) mod operations;

// type PrimaryKey = i32;

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
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub access_token: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "bet")]
pub struct BetModel {
    pub user_id: String,
    pub amount: BigDecimal,
    pub date: NaiveDate,
}
