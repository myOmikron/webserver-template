//! Custom Json extractor

use axum::extract::FromRequest;
use axum::response::IntoResponse;
use schemars::JsonSchema;
use serde::Serialize;
use swaggapi::as_responses::AsResponses;
use swaggapi::internals::SchemaGenerator;
use swaggapi::re_exports::openapiv3::Responses;

use crate::http::common::errors::ApiError;

/// Custom extractor that is based on [axum::Json], but its errors
/// will be converted to [ApiError]
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

// We implement `IntoResponse` for our extractor so it can be used as a response
impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl<T: Serialize + JsonSchema> AsResponses for Json<T> {
    fn responses(gen: &mut SchemaGenerator) -> Responses {
        swaggapi::as_responses::ok_json::<T>(gen)
    }
}
