//! Provides access to the database.
use chrono::NaiveDateTime;
pub use client::PgDbClient;
use sqlx::types::BigDecimal;

mod client;
pub(crate) mod operations;

/// Snowflake ID.
type Snowflake = String;

#[derive(Debug, PartialEq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
pub enum BookmakerScope {
    Bookmaker,
    Exchange,
}

#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "bookmakers")]
pub struct BookmakerModel {
    pub id: Snowflake,
    pub label: String,
    pub slug: String,
    pub url: String,
    pub scope: BookmakerScope,
    pub created_at: NaiveDateTime,
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
    pub bookmaker_id: BookmakerScope,
    pub amount: BigDecimal,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct BetListModel {
    pub id: Snowflake,
    pub amount: BigDecimal,
    pub bookmaker: String,
    pub monthly_profit: Option<BigDecimal>,
    pub total_profit: Option<BigDecimal>,
    pub created_at: NaiveDateTime,
}
