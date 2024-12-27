use super::{BetListModel, BookmakerModel, BookmakerScope, UserModel};
use sqlx::{types::BigDecimal, PgExecutor, PgPool};

pub(crate) async fn create_user(
    executor: impl PgExecutor<'_>,
    data: &UserModel,
) -> anyhow::Result<()> {
    // language=PostgreSQL
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, avatar, access_token)
        VALUES ($1, $2, $3, $4)
        "#,
        data.id,
        data.username,
        data.avatar,
        data.access_token
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub(crate) async fn get_user(
    executor: impl PgExecutor<'_>,
    id: &str,
) -> anyhow::Result<Option<UserModel>> {
    // language=PostgreSQL
    let row = sqlx::query!(
        r#"
        SELECT id, username, avatar, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(executor)
    .await?;

    Ok(row.map(|row| UserModel {
        id: row.id,
        username: row.username,
        avatar: row.avatar,
        created_at: row.created_at,
        access_token: None,
    }))
}

pub(crate) async fn create_bet(
    pool: &PgPool,
    bookmaker_id: &str,
    amount: BigDecimal,
    user_id: &str,
) -> anyhow::Result<String> {
    let mut tx = pool.begin().await?;

    // language=PostgreSQL
    let bet_id = sqlx::query_scalar!(
        r#"
        INSERT INTO bets (id, bookmaker_id, amount)
        VALUES (id_generator(), $1, $2)
        RETURNING id
        "#,
        bookmaker_id as _,
        amount,
    )
    .fetch_one(&mut *tx)
    .await?;

    // language=PostgreSQL
    sqlx::query!(
        r#"
        INSERT INTO user_bets (user_id, bet_id)
        VALUES ($1, $2)
        "#,
        user_id,
        bet_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(bet_id)
}

pub(crate) async fn get_bookmaker(
    executor: impl PgExecutor<'_>,
    bookmaker_id: &str,
) -> anyhow::Result<BookmakerModel> {
    let row = sqlx::query_as!(
        BookmakerModel,
        r#"
    SELECT
        id,
        label,
        slug,
        url,
        scope as "scope: BookmakerScope",
        created_at
    FROM bookmakers
    WHERE id = $1
        "#,
        bookmaker_id
    )
    .fetch_one(executor)
    .await?;

    Ok(row)
}

pub(crate) async fn get_bets(
    executor: impl PgExecutor<'_>,
    user_id: &str,
) -> anyhow::Result<Vec<BetListModel>> {
    let rows = sqlx::query_as!(
        BetListModel,
        r#"
    SELECT
        b.id,
        b.amount,
        bm.label as bookmaker,
        sum(b.amount) OVER (ORDER BY b.created_at) as monthly_profit,
        sum(b.amount) OVER () as total_profit,
        b.created_at
    FROM bets b
    JOIN user_bets ub ON b.id = ub.bet_id
    JOIN bookmakers bm ON b.bookmaker_id = bm.id
    WHERE ub.user_id = $1
        "#,
        user_id
    )
    .fetch_all(executor)
    .await?;

    Ok(rows)
}
