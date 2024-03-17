use crate::types::VersionInfo;

use crate::build;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

/// Create info routes.
/// Helper method to easily nest all info routes under common prefix.
pub fn info_routes() -> Router {
    Router::new()
        .route("/version", get(version))
        .route("/healthcheck", get(healthcheck))
}

/// Return OK as a simple healthcheck.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/info/healthcheck",
    responses((status = 200, description = "Simple OK response as healthcheck"))
)]
pub async fn healthcheck() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

/// Return version information for API.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/info/version",
    responses(
        (status = 200, body = [VersionInfo], description = "Version information")
    )
)]
pub async fn version() -> (StatusCode, Json<VersionInfo>) {
    tracing::info!("Version: {}", build::PKG_VERSION);
    (StatusCode::OK, Json(VersionInfo::from_build_info()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_router;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::Value;

    #[tokio::test]
    async fn test_healthcheck() {
        let app = build_router();
        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let response = server.get("/info/healthcheck").await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_version() {
        let app = build_router();
        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        let response = server.get("/info/version").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let body: Value = response.json();

        assert_eq!(body["name"], build::PROJECT_NAME);
        assert_eq!(body["version"], build::PKG_VERSION);
        assert_eq!(body["build_time"], build::BUILD_TIME_3339);
        assert_eq!(body["branch"], build::BRANCH);
        assert_eq!(body["commit"], build::COMMIT_HASH);
        assert_eq!(body["commit_time"], build::COMMIT_DATE);
        assert_eq!(body["build_os"], build::BUILD_OS);
        assert_eq!(body["rust_version"], build::RUST_VERSION);
        assert_eq!(body["rust_channel"], build::RUST_CHANNEL);
    }
}
