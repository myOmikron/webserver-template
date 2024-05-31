//! The handler for the users

use axum::Json;
use swaggapi::get;
use tracing::instrument;

use crate::http::common::errors::ApiResult;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::users::schema::FullUser;

/// Retrieve the currently logged-in user
#[get("/me")]
#[instrument(skip_all)]
pub async fn get_me(SessionUser(user): SessionUser) -> ApiResult<Json<FullUser>> {
    Ok(Json(FullUser {
        uuid: user.uuid,
        display_name: user.display_name,
    }))
}
