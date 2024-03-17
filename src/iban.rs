use crate::types::{IbanInfo, IbanResponse, MessageResponse};

use axum::response::IntoResponse;
use axum::routing::get;
use axum::{extract::Path, Router};
use iban::*;

/// Create iban routes.
/// Helper method to easily nest all iban routes under common prefix.
pub fn iban_routes() -> Router {
    Router::new().route("/:iban", get(iban))
}

/// Get iban info if valid, an error otherwise.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/iban/{iban}",
    responses(
        (status = 200, body = [IbanInfo], description = "Valid IBAN"),
        (status = 400, body = [MessageResponse], description = "Invalid IBAN")
    )
)]
pub async fn iban(Path(account): Path<String>) -> impl IntoResponse {
    tracing::info!("IBAN: {}", account);
    match account.parse::<Iban>() {
        Ok(valid_iban) => IbanResponse::Info(IbanInfo {
            iban: valid_iban.to_string(),
            bban: valid_iban.bban().to_string(),
            check_digits: valid_iban.check_digits(),
            bank_identifier: valid_iban.bank_identifier().map(str::to_string),
            branch_identifier: valid_iban.branch_identifier().map(str::to_string),
            country_code: valid_iban.country_code().to_string(),
        }),
        Err(err) => IbanResponse::Error(MessageResponse {
            message: format!("Invalid IBAN: {}", err),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::build_router;
    use crate::types::{IbanInfo, MessageResponse};
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_iban_valid() {
        let app = build_router();
        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Get the request.
        let response = server.get("/iban/DE44500105175407324931").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let iban_info = response.json::<IbanInfo>();
        assert_eq!(iban_info.country_code, "DE");
        assert_eq!(iban_info.check_digits, 44);
        assert_eq!(iban_info.bban, "500105175407324931");
        assert_eq!(iban_info.bank_identifier, Some(String::from("50010517")));
        assert_eq!(iban_info.branch_identifier, None);
    }

    #[tokio::test]
    async fn test_iban_invalid() {
        let app = build_router();
        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Get the request.
        let response = server.get("/iban/XX44500105175407324931").await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        let iban_response = response.json::<MessageResponse>();
        assert_eq!(
            iban_response.message,
            "Invalid IBAN: the string does not follow the base IBAN rules"
        );
    }
}
