use crate::controllers;
use crate::controllers::util::RequestPartsExt;
use crate::database::{PgDbClient, UserModel};
use crate::middleware::log_request::RequestLogExt;
use crate::middleware::session::RequestSession;
use crate::util::errors::forbidden;
use crate::util::errors::{internal, AppResult};
use http::request::Parts;
// use axum::RequestPartsExt;
// use crate::util::token::HashedToken;
use tracing::instrument;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthCheck {
    allow_token: bool,
    endpoint_scope: Option<()>,
}

#[allow(dead_code)]
impl AuthCheck {
    #[must_use]
    // #[must_use] can't be applied in the `Default` trait impl
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self {
            allow_token: true,
            endpoint_scope: None,
        }
    }

    #[must_use]
    pub fn only_cookie() -> Self {
        Self {
            allow_token: false,
            endpoint_scope: None,
        }
    }

    // pub fn with_endpoint_scope(&self, endpoint_scope: EndpointScope) -> Self {
    //     Self {
    //         allow_token: self.allow_token,
    //         endpoint_scope: Some(endpoint_scope),
    //         bot_id: self.bot_id.clone(),
    //     }
    // }

    #[instrument(name = "auth.check", skip_all)]
    pub async fn check(&self, request: &Parts, db: &PgDbClient) -> AppResult<Authentication> {
        let auth = authenticate(request, db).await?;

        // if let Some(token) = auth.api_token() {
        //     if !self.allow_token {
        //         let error_message =
        //             "API Token authentication was explicitly disallowed for this API";
        //         request.request_log().add("cause", error_message);
        //
        //         return Err(forbidden(
        //             "this action can only be performed on the dbots.fun website",
        //         ));
        //     }
        //
        //     if !self.endpoint_scope_matches(token.endpoint_scopes.as_ref()) {
        //         let error_message = "Endpoint scope mismatch";
        //         request.request_log().add("cause", error_message);
        //
        //         return Err(forbidden(
        //             "this token does not have the required permissions to perform this action",
        //         ));
        //     }
        // }

        Ok(auth)
    }

    fn endpoint_scope_matches(&self, token_scopes: Option<&Vec<()>>) -> bool {
        match (&token_scopes, &self.endpoint_scope) {
            // The token is a legacy token.
            (None, _) => true,

            // The token is NOT a legacy token, and the endpoint only allows legacy tokens.
            (Some(_), None) => false,

            // The token is NOT a legacy token, and the endpoint allows a certain endpoint scope or a legacy token.
            (Some(token_scopes), Some(endpoint_scope)) => token_scopes.contains(endpoint_scope),
        }
    }
}

#[derive(Debug)]
pub enum Authentication {
    Cookie(CookieAuthentication),
    // Token(TokenAuthentication),
}

#[derive(Debug)]
pub struct CookieAuthentication {
    user: UserModel,
}

// #[derive(Debug)]
// pub struct TokenAuthentication {
//     token: ApiToken,
//     user: User,
// }

impl Authentication {
    pub fn user_id(&self) -> String {
        self.user().id.clone()
    }

    // pub fn api_token_id(&self) -> Option<i32> {
    //     self.api_token().map(|token| token.id)
    // }
    //
    // pub fn api_token(&self) -> Option<&ApiToken> {
    //     match self {
    //         Authentication::Token(token) => Some(&token.token),
    //         _ => None,
    //     }
    // }

    pub fn user(&self) -> &UserModel {
        match self {
            Authentication::Cookie(cookie) => &cookie.user,
            // Authentication::Token(token) => &token.user,
        }
    }
}

#[instrument(skip_all)]
async fn authenticate_via_cookie<T: RequestPartsExt>(
    req: &T,
    db: &PgDbClient,
) -> AppResult<Option<CookieAuthentication>> {
    let user_id_from_session = req
        .session()
        .get("user_id")
        .and_then(|s| s.parse::<String>().ok());

    let Some(id) = user_id_from_session else {
        return Ok(None);
    };

    let user = db.get_user(id.as_str()).await.map_err(|err| {
        req.request_log().add("cause", err);
        internal("user_id from cookie not found in database")
    })?;

    req.request_log().add("uid", id);

    Ok(Some(CookieAuthentication { user }))
}

// #[instrument(skip_all)]
// async fn authenticate_via_token<T: RequestPartsExt>(
//     req: &T,
//     db: &mut PgDbClient,
// ) -> AppResult<Option<TokenAuthentication>> {
//     let maybe_authorization = req
//         .headers()
//         .get(header::AUTHORIZATION)
//         .and_then(|h| h.to_str().ok());
//
//     let Some(header_value) = maybe_authorization else {
//         return Ok(None);
//     };
//
//     let token =
//         HashedToken::parse(header_value).map_err(|_| InsecurelyGeneratedTokenRevoked::boxed())?;
//
//     let token = ApiToken::find_by_api_token(db, &token).map_err(|e| {
//         let cause = format!("invalid token caused by {e}");
//         req.request_log().add("cause", cause);
//
//         forbidden("authentication failed")
//     })?;
//
//     let user_id = token.user_id.clone();
//
//     let user = User::find(db, user_id.as_str()).map_err(|err| {
//         req.request_log().add("cause", err);
//         internal("user_id from token not found in database")
//     })?;
//
//     req.request_log().add("uid", user_id);
//     req.request_log().add("tokenid", token.id);
//
//     Ok(Some(TokenAuthentication { user, token }))
// }

#[instrument(skip_all)]
async fn authenticate(req: &Parts, db: &PgDbClient) -> AppResult<Authentication> {
    controllers::util::verify_origin(req)?;

    match authenticate_via_cookie(req, db).await {
        Ok(None) => {}
        Ok(Some(auth)) => return Ok(Authentication::Cookie(auth)),
        Err(err) => return Err(err),
    }

    // match authenticate_via_token(req, db) {
    //     Ok(None) => {}
    //     Ok(Some(auth)) => return Ok(Authentication::Token(auth)),
    //     Err(err) => return Err(err),
    // }

    // Unable to authenticate the user
    let cause = "no cookie session or auth header found";
    req.request_log().add("cause", cause);

    return Err(forbidden("this action requires authentication"));
}
