use crate::config;
use crate::database::PgDbClient;
use axum::extract::{FromRef, FromRequestParts, State};
use derive_more::Deref;
use reqwest::Client as ReqwestClient;
use std::sync::Arc;

pub struct App {
    pub config: Arc<config::Server>,
    pub db: Arc<PgDbClient>,
    pub http: ReqwestClient,
}

impl App {
    pub fn new(config: config::Server, db: PgDbClient) -> Self {
        let http = ReqwestClient::new();

        Self {
            config: Arc::new(config),
            db: Arc::new(db),
            http,
        }
    }

    /// A unique key to generate signed cookies
    pub fn session_key(&self) -> &cookie::Key {
        &self.config.session_key
    }
}

#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl FromRef<AppState> for cookie::Key {
    fn from_ref(app: &AppState) -> Self {
        app.session_key().clone()
    }
}
