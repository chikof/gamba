use std::sync::Arc;

use axum::Json;
use http::request::Parts;
use serde_json::{json, Value};

use crate::app::AppState;
use crate::auth::AuthCheck;
use crate::util::errors::AppResult;

/// Handles the `/me/bets` route.
pub async fn list_bets(app: AppState, req: Parts) -> AppResult<Json<Value>> {
    let db = Arc::clone(&app.db);

    // Make sure the user is authenticated.
    let auth = AuthCheck::only_cookie().check(&req, &db).await?;

    // Get the user ID from the authentication token.
    let user_id = auth.user_id();

    // Get the list of bets for the user.
    let bets = db.get_bets(&user_id).await?;

    Ok(Json(json!({
        "user_id": user_id,
        "bets": bets
    })))
}
