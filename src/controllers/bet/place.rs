use crate::app::AppState;
use crate::auth::AuthCheck;
use crate::util::errors::AppResult;
use axum::Json;
use http::request::Parts;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct PlaceBet {
    // TODO: Convert to decimals and validate the amount.
    pub amount: String,
    // Seed a list of casinos in the database.
    pub casino: String,
}

/// Handles the requests for placing a bet.
/// `POST /bet/place`
pub async fn place(app: AppState, req: Parts, Json(bet): Json<PlaceBet>) -> AppResult<Json<Value>> {
    let db = Arc::clone(&app.db);

    // Make sure the user is authenticated.
    let auth = AuthCheck::only_cookie().check(&req, &db).await?;

    // Get the user ID from the authentication token.
    let user_id = auth.user_id();

    // Create the bet in the database.
    // If the bet is successful, return the bet ID.
    let bet_id = db.create_bet(&bet.amount, &bet.casino, &user_id).await?;

    Ok(Json(json!({
        "id": bet_id
    })))
}
