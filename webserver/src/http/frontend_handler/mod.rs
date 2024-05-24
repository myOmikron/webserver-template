//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use axum::Extension;
use axum::Router;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPageBuilder;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::auth_required;
use crate::utils::oidc::OidcClient;

pub mod auth;
pub mod oidc;
pub mod users;

/// The Swagger definition for the frontend api v1
pub static FRONTEND_API_V1: SwaggapiPageBuilder =
    SwaggapiPageBuilder::new().filename("frontend_v1.json");

/// Create the router for the Frontend API
pub fn get_routes(oidc_client: Option<OidcClient>) -> Router {
    let mut api_context = ApiContext::new();

    if let Some(oidc_client) = oidc_client {
        api_context = api_context.nest(
            "/api/frontend/v1/oidc",
            ApiContext::new()
                .handler(oidc::handler::login)
                .handler(oidc::handler::finish_login)
                .route_layer(ServiceBuilder::new().layer(Extension(oidc_client))),
        );
    }

    api_context
        .nest(
            "/api/frontend/v1/auth",
            ApiContext::new()
                .handler(auth::handler::login)
                .route_layer(ServiceBuilder::new().concurrency_limit(10)),
        )
        .nest(
            "/api/frontend/v1",
            ApiContext::new()
                .handler(users::handler::get_me)
                .handler(auth::handler::logout)
                .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_required))),
        )
        .into()
}
