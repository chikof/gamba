use super::UserModel;
use sqlx::{types::BigDecimal, PgExecutor, PgPool, Transaction};

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
    casino: &str,
    amount: BigDecimal,
    user_id: &str,
) -> anyhow::Result<String> {
    let mut tx: Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    // language=PostgreSQL
    let bet_id = sqlx::query_scalar!(
        r#"
        INSERT INTO bets (id, casino, amount)
        VALUES (id_generator(), $1, $2)
        RETURNING id
        "#,
        casino,
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
