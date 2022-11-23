pub mod ethereum;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::{json::Json, Serialize},
    Request, Response,
};
use std::io::Cursor;

use eyre::Result;

/// The API response.
pub enum ApiResponse<ResponseType>
where
    ResponseType: Serialize,
{
    /// The API response is a success.
    Success(ResponseType),
    /// The API response is an error.
    Error(ApiError),
}

/// The API error.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    /// The error message.
    error_message: String,
}

impl<'a, ResponseType> Responder<'a, 'static> for ApiResponse<ResponseType>
where
    ResponseType: Serialize,
{
    /// Respond to the request.
    /// # Arguments
    /// * `request` - The request.
    /// # Returns
    /// `Ok(response)` - The response.
    /// # Errors
    /// If the response cannot be serialized.
    fn respond_to(self, req: &'a Request<'_>) -> response::Result<'static> {
        match self {
            ApiResponse::Success(response) => Json(response).respond_to(req),
            ApiResponse::Error(error) => {
                let body = serde_json::to_string(&error).unwrap();
                Ok(Response::build()
                    .header(ContentType::JSON)
                    .sized_body(body.len(), Cursor::new(body))
                    .status(Status::InternalServerError)
                    .finalize())
            }
        }
    }
}

impl<ResponseType> ApiResponse<ResponseType>
where
    ResponseType: Serialize,
{
    /// Create a new success API response.
    /// # Arguments
    /// * `response` - The response.
    /// # Returns
    /// `ApiResponse::Success(response)` - The success API response.
    pub fn success(response: ResponseType) -> ApiResponse<ResponseType> {
        ApiResponse::Success(response)
    }

    /// Create a new error API response.
    /// # Arguments
    /// * `error_message` - The error message.
    /// # Returns
    /// `ApiResponse::Error(error)` - The error API response.
    pub fn error(error_message: String) -> ApiResponse<ResponseType> {
        ApiResponse::Error(ApiError { error_message })
    }

    /// Create a new error API response from a `Result`.
    pub fn from_result(result: Result<ResponseType>) -> ApiResponse<ResponseType> {
        match result {
            Ok(response) => ApiResponse::success(response),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }
}
