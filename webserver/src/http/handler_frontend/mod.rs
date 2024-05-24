//! Parts of the http api for the frontend
//!
//! This included the router as well as the handlers and schemas

use axum::Extension;
use axum::Router;
use openidconnect::core::CoreClient;
use swaggapi::ApiContext;
use swaggapi::SwaggapiPageBuilder;
use tower::ServiceBuilder;

use crate::http::middlewares::auth_required::auth_required;

pub mod auth;
pub mod oidc;
pub mod users;

/// The Swagger definition for the frontend api v1
pub static FRONTEND_API_V1: SwaggapiPageBuilder =
    SwaggapiPageBuilder::new().filename("frontend_v1.json");

/// Create the router for the Frontend API
pub fn initialize(oidc_client: Option<CoreClient>) -> ApiContext<Router> {
    let oidc_context = if let Some(oidc_client) = oidc_client {
        ApiContext::new()
            .handler(oidc::handler::login)
            .handler(oidc::handler::finish_login)
            .route_layer(ServiceBuilder::new().layer(Extension(oidc_client)))
    } else {
        ApiContext::new()
    };

    ApiContext::new().nest(
        "/v1",
        ApiContext::new()
            .nest("/oidc", oidc_context)
            .nest(
                "/auth",
                ApiContext::new()
                    .handler(auth::handler::login)
                    .route_layer(ServiceBuilder::new().concurrency_limit(10)),
            )
            .merge(
                ApiContext::new()
                    .handler(users::handler::get_me)
                    .handler(auth::handler::logout)
                    .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_required))),
            ),
    )
}
