use std::ops::Deref;
use std::sync::Arc;

use crate::config;
use crate::database::PgDbClient;
use axum::extract::{FromRequestParts, State};

pub struct App {
    pub config: Arc<config::Server>,
    pub db: Arc<PgDbClient>,
}

impl App {
    pub fn new(config: config::Server, db: PgDbClient) -> Self {
        Self {
            config: Arc::new(config),
            db: Arc::new(db),
        }
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

// deref so you can still access the inner fields easily
impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
