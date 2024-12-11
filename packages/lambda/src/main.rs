mod cognito;
mod config;
mod handlers;

use std::sync::Arc;

use cognito::CognitoService;
use config::Config;
use lambda_http::run;
use lambda_http::service_fn;
use lambda_http::tracing;
use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Request;
use lambda_http::Response;
use url::Url;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  let event = Arc::new(event);
  let url = Url::parse(&event.uri().to_string())?;
  let config = Config::from_env()?;

  let cognito_service = CognitoService {
    cognito_origin: config.cognito_origin.clone(),
    token_endpoint: config.token_endpoint.clone(),
    local_origin: config.local_origin.clone(),
    cognito_client_id: config.cognito_client_id.clone(),
    cognito_secret: config.cognito_secret.clone(),
  };
  let ctx = handlers::Context {
    req: event.clone(),
    config: config,
    query: handlers::QueryString::new(event.clone()),
    cognito_service,
  };

  match url.path() {
    "/api/ping" => handlers::ping_any::handler(ctx).await,
    "/api/auth/login" => handlers::auth_login_get::handler(ctx).await,
    "/api/auth/login/callback" => handlers::auth_login_callback_get::handler(ctx).await,
    "/api/auth/logout" => handlers::auth_logout_get::handler(ctx).await,
    "/api/auth/logout/callback" => handlers::auth_logout_callback_get::handler(ctx).await,
    "/api/auth/refresh" => handlers::auth_refresh_get::handler(ctx).await,
    "/api/auth/validate" => handlers::auth_validate_get::handler(ctx).await,
    _ => handlers::not_found_any::handler(ctx).await,
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing::init_default_subscriber();
  run(service_fn(function_handler)).await
}
