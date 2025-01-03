use crate::controllers::util::RequestPartsExt;
use crate::util::base64::{base64_decode, base64_encode};
use axum::extract::{Extension, FromRequestParts, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::SignedCookieJar;
use cookie::time::Duration;
use cookie::{Cookie, SameSite};
use crates_io_env_vars::required_var;
use derive_more::Deref;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

static COOKIE_NAME: &str = "gamba_session";
static MAX_AGE_DAYS: i64 = 90;

#[derive(Clone, FromRequestParts, Deref)]
#[from_request(via(Extension))]
pub struct SessionExtension(Arc<RwLock<Session>>);

impl SessionExtension {
    fn new(session: Session) -> Self {
        Self(Arc::new(RwLock::new(session)))
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let session = self.read();
        session.data.get(key).cloned()
    }

    pub fn insert(&self, key: String, value: String) -> Option<String> {
        let mut session = self.write();
        session.dirty = true;
        session.data.insert(key, value)
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        let mut session = self.write();
        session.dirty = true;
        session.data.remove(key)
    }
}

pub async fn attach_session(jar: SignedCookieJar, mut req: Request, next: Next) -> Response {
    // Decode session cookie
    let data = jar.get(COOKIE_NAME).map(decode).unwrap_or_default();

    // Save decoded session data in request extension,
    // and keep an `Arc` clone for later
    let session = SessionExtension::new(Session::new(data));
    req.extensions_mut().insert(session.clone());

    // Process the request
    let response = next.run(req).await;

    // Get the domain where the cookie is going to be created
    let domain = required_var("COOKIE_DOMAIN").unwrap_or("localhost".to_string());

    // Check if the session data was mutated
    let session = session.read();
    if session.dirty {
        // Return response with additional `Set-Cookie` header
        let encoded = encode(&session.data);
        let cookie = Cookie::build((COOKIE_NAME, encoded))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(Duration::days(MAX_AGE_DAYS))
            .domain(domain)
            .path("/");

        (jar.add(cookie), response).into_response()
    } else {
        response
    }
}

/// Request extension to access the session data
pub struct Session {
    data: HashMap<String, String>,
    dirty: bool,
}

impl Session {
    fn new(data: HashMap<String, String>) -> Self {
        Self { data, dirty: false }
    }
}

pub trait RequestSession {
    fn session(&self) -> &SessionExtension;
}

impl<T: RequestPartsExt> RequestSession for T {
    fn session(&self) -> &SessionExtension {
        self.extensions()
            .get::<SessionExtension>()
            .expect("missing cookie session")
    }
}

pub fn decode(cookie: Cookie<'_>) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    let bytes = base64_decode(cookie.value()).unwrap_or_default();
    let mut parts = bytes.split(|&x| x == 0xff);
    while let (Some(key), Some(value)) = (parts.next(), parts.next()) {
        if key.is_empty() {
            break;
        }
        if let (Ok(key), Ok(value)) = (std::str::from_utf8(key), std::str::from_utf8(value)) {
            ret.insert(key.to_string(), value.to_string());
        }
    }
    ret
}

pub fn encode(h: &HashMap<String, String>) -> String {
    let mut ret = Vec::new();
    for (i, (k, v)) in h.iter().enumerate() {
        if i != 0 {
            ret.push(0xff)
        }
        ret.extend(k.bytes());
        ret.push(0xff);
        ret.extend(v.bytes());
    }
    while ret.len() * 8 % 6 != 0 {
        ret.push(0xff);
    }
    base64_encode(&ret[..])
}
