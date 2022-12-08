pub mod ethereum;
pub mod starknet;

use eyre::Result;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::{json::Json, Serialize},
    Request, Response,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::{openapi3::Responses, Map},
    response::OpenApiResponderInner,
    OpenApiError,
};
use schemars::JsonSchema;
use std::io::Cursor;

/// The API response.
pub enum ApiResponse<ResponseType>
where
    ResponseType: Serialize + JsonSchema,
{
    /// The API response is a success.
    Success(ResponseType),
    /// The API response is an error.
    Error(ApiError),
}

/// The API error.
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    /// The error message.
    error_message: String,
}

impl<'a, ResponseType> Responder<'a, 'static> for ApiResponse<ResponseType>
where
    ResponseType: Serialize + JsonSchema,
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
    ResponseType: Serialize + JsonSchema,
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

impl<ResponseType> OpenApiResponderInner for ApiResponse<ResponseType>
where
    ResponseType: Serialize + JsonSchema,
{
    /// Generate the OpenAPI error responses.
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)\n\
                The request given is wrongly formatted or data asked could not be fulfilled. \
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [404 Not Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)\n\
                This response is given when you request a page that does not exists.\
                "
                .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [422 Unprocessable Entity](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422)\n\
                This response is given when you request body is not correctly formatted. \
                ".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)\n\
                This response is given when something wend wrong on the server. \
                ".to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}
