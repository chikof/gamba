use crate::app::AppState;
use crate::auth::AuthCheck;
use crate::database::UserModel;
use crate::util::errors::AppResult; // views::{EncodableMe, EncodablePrivateUser},
use axum::http::request::Parts;
use axum::Json;
use std::sync::Arc;

/// Handles the `GET /me` route.
pub async fn me(app: AppState, req: Parts) -> AppResult<Json<UserModel>> {
    let db = Arc::clone(&app.db);
    let user_id = AuthCheck::only_cookie().check(&req, &db).await?.user_id();
    let user_id = user_id.as_str();

    let user = db.get_user(user_id).await?;

    Ok(Json(user))
}
