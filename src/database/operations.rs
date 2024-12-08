use super::{BetModel, UserModel};
use sqlx::PgExecutor;

pub(crate) async fn create_user(
    executor: impl PgExecutor<'_>,
    data: &UserModel,
) -> anyhow::Result<()> {
    // language=PostgreSQL
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, avatar)
        VALUES ($1, $2, $3)
        "#,
        data.id,
        data.username,
        data.avatar
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
    }))
}

pub(crate) async fn create_bet(
    executor: impl PgExecutor<'_>,
    data: &BetModel,
) -> anyhow::Result<()> {
    // language=PostgreSQL
    sqlx::query!(
        r#"
        INSERT INTO bets (user_id, amount, date)
        VALUES ($1, $2, $3)
        "#,
        data.user_id,
        data.amount,
        data.date
    )
    .execute(executor)
    .await?;

    Ok(())
}
