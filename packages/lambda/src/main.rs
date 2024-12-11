use lambda_http::run;
use lambda_http::service_fn;
use lambda_http::tracing;
use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Request;
use lambda_http::Response;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
  if event.uri() == "/api/ping" {
    Ok(Response::builder()
      .status(200)
      .header("Content-Type", "application/json")
      .body("{\"message\":\"pong\"}".into())
      .map_err(Box::new)?)
  } else {
    Ok(Response::builder()
      .status(200)
      .header("Content-Type", "application/json")
      .body("{\"message\":\"hello world\"}".into())
      .map_err(Box::new)?)
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing::init_default_subscriber();
  run(service_fn(function_handler)).await
}
