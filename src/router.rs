use crate::app::AppState;
use crate::controllers::*;
use crate::util::errors::not_found;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Router;
use http::{Method, StatusCode};

pub fn build_axum_router(state: AppState) -> Router<()> {
    let router = Router::new()
        .route("/", get(site_metadata::metadata))
        // user routes
        .route("/me", get(user::me::me))
        .route("/me/bets", get(bet::list::list_bets))
        .route("/private/session/login", get(user::session::login))
        .route("/private/session/authorize", get(user::session::authorize))
        .route("/private/session", delete(user::session::logout))
        // bet routes
        .route("/bet/place", post(bet::place::place));

    router
        .fallback(|method: Method| async move {
            match method {
                Method::HEAD => StatusCode::NOT_FOUND.into_response(),
                _ => not_found().into_response(),
            }
        })
        .with_state(state)
}
