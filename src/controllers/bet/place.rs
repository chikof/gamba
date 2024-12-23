use crate::app::AppState;
use crate::auth::AuthCheck;
use crate::util::errors::AppResult;
use axum::Json;
use axum_valid::Valid;
use http::request::Parts;
use serde_json::{json, Value};
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PlaceBet {
    pub amount: f32,
    #[validate(length(equal = 19, message = "Enter a valid bookmaker ID"))]
    pub bookmaker_id: String,
}

/// Handles the requests for placing a bet.
/// `POST /bet/place`
pub async fn place(
    app: AppState,
    req: Parts,
    Valid(Json(bet)): Valid<Json<PlaceBet>>,
) -> AppResult<Json<Value>> {
    let db = Arc::clone(&app.db);

    // Make sure the user is authenticated.
    let auth = AuthCheck::only_cookie().check(&req, &db).await?;

    // Get the user ID from the authentication token.
    let user_id = auth.user_id();

    // Create the bet in the database.
    // If the bet is successful, return the bet ID.
    let bet_id = db
        .create_bet(&bet.amount.to_string(), &bet.bookmaker_id, &user_id)
        .await?;

    // Get the bookmaker metadata
    let bookmaker = db.get_bookmaker(&bet.bookmaker_id).await?;

    Ok(Json(json!({
        "bet_id": bet_id,
        "bookmaker": bookmaker
    })))
}
