mod config;
mod handlers;

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
  let url = Url::parse(&event.uri().to_string())?;
  let config = Config::from_env()?;

  match url.path() {
    "/api/ping" => handlers::ping_any::handler(event).await,
    "/api/auth/login" => handlers::auth_login_get::handler(event, url, &config).await,
    "/api/auth/login/callback" => handlers::auth_login_callback_get::handler(event).await,
    "/api/auth/logout" => handlers::auth_logout_get::handler(event).await,
    "/api/auth/logout/callback" => handlers::auth_logout_callback_get::handler(event).await,
    "/api/auth/refresh" => handlers::auth_refresh_get::handler(event).await,
    "/api/auth/validate" => handlers::auth_validate_get::handler(event).await,
    _ => handlers::not_found_any::handler(event).await,
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing::init_default_subscriber();
  run(service_fn(function_handler)).await
}
