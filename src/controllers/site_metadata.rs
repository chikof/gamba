use axum::response::IntoResponse;
use axum_extra::json;

// This file is generated by build.rs
include!(concat!(env!("OUT_DIR"), "/version.rs"));

/// Returns the JSON representation of the site metadata.
///
/// The sha is contained in the `SOURCE_COMMIT` environment variable.
/// If `SOURCE_COMMIT` is not set, the sha will be "unknown".
/// The branch is contained in the `COOLIFY_BRANCH` environment variable.
/// If `COOLIFY_BRANCH` is not set, the branch will be "unknown".
pub async fn metadata() -> impl IntoResponse {
    let deployed_sha = dotenvy::var("SOURCE_COMMIT").unwrap_or_else(|_| String::from("unknown"));
    let branch = dotenvy::var("COOLIFY_BRANCH").unwrap_or_else(|_| String::from("unknown"));

    json!({
        "name": format!("gamba:{branch}"),
        "version": VERSION,
        "deployed_sha": &deployed_sha[..],
        "commit": &deployed_sha[..],
    })
}