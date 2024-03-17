use crate::build;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use utoipa::ToSchema;

use std::fmt;

/// Logging level CLI parameter
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IbanInfo {
    #[schema(example = "DE44500105175407324931")]
    pub iban: String,
    #[schema(example = "500105175407324931")]
    pub bban: String,
    #[schema(example = "44")]
    pub check_digits: u8,
    #[schema(example = "50010517")]
    pub bank_identifier: Option<String>,
    pub branch_identifier: Option<String>,
    #[schema(example = "DE")]
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum IbanResponse {
    Info(IbanInfo),
    Error(MessageResponse),
}

impl IntoResponse for IbanResponse {
    fn into_response(self) -> Response {
        match self {
            IbanResponse::Info(info) => (StatusCode::OK, Json(info)).into_response(),
            IbanResponse::Error(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }
}

/// Simple response with a message
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    /// Message can be either information or an error message
    #[schema(example = "Invalid IBAN")]
    pub message: String,
}

/// API version information.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct VersionInfo {
    #[schema(example = "axum-iban")]
    pub name: String,
    #[schema(example = "0.5.0")]
    pub version: String,
    #[schema(example = "2024-02-14 14:42:35 +02:00")]
    pub build_time: String,
    #[schema(example = "main")]
    pub branch: String,
    #[schema(example = "ee9ec805f61944653a56a7e429b2fad03232be49")]
    pub commit: String,
    #[schema(example = "2024-02-14 12:42:18 +00:00")]
    pub commit_time: String,
    #[schema(example = "macos-aarch64")]
    pub build_os: String,
    #[schema(example = "rustc 1.76.0 (07dca489a 2024-02-04)")]
    pub rust_version: String,
    #[schema(example = "stable-aarch64-apple-darwin")]
    pub rust_channel: String,
}

/// Custom error type that enables using anyhow error handling in routes.
/// This is used for server-side errors and returns status code 500 with the error message.
pub struct ServerError(anyhow::Error);

// Tell axum how to convert `ServerError` into a response.
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", self.0))).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>`
// to turn them into `Result<_, ServerError>`.
// This way we don't need to do that manually.
impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// Implement `Display` for `MinMax`.
impl fmt::Display for MessageResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.message)
    }
}
impl LogLevel {
    pub fn to_filter(&self) -> LevelFilter {
        match self {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

impl VersionInfo {
    pub fn from_build_info() -> VersionInfo {
        VersionInfo {
            name: build::PROJECT_NAME.to_string(),
            version: build::PKG_VERSION.to_string(),
            build_time: build::BUILD_TIME_3339.to_string(),
            branch: build::BRANCH.to_string(),
            commit: build::COMMIT_HASH.to_string(),
            commit_time: build::COMMIT_DATE.to_string(),
            build_os: build::BUILD_OS.to_string(),
            rust_version: build::RUST_VERSION.to_string(),
            rust_channel: build::RUST_CHANNEL.to_string(),
        }
    }
}
