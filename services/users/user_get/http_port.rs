use crate::domain::user_get_core;

use error::HexagonalError;
use http::{Error, Response, StatusCode};
use http_apigw_adaptor::HttpPortRequest;
use models::models::user::UserRepositoryPort;

pub async fn user_get_get_http_port<T1: UserRepositoryPort>(
    user_repository_port: &T1,
    http_request: HttpPortRequest,
) -> Result<Response<String>, Error> {
    let email = match http_request.query_string_parameters.first("email") {
        Some(value) => value,
        None => {
            let err = HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "email is required".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }
    };
    match user_get_core(user_repository_port, &email.to_string()).await {
        Ok(user) => match user {
            Some(result) => {
                let resp = Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&result).unwrap());
                Ok(resp.unwrap())
            }
            None => {
                let resp = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("content-type", "application/json")
                    .body("".to_string());
                Ok(resp.unwrap())
            }
        },
        Err(err) => Ok(err.compile_to_http_response()),
    }
}
