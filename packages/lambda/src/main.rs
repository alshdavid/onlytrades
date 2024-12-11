use lambda_http::run;
use lambda_http::service_fn;
use lambda_http::tracing;
use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Request;
use lambda_http::Response;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
  let resp = Response::builder()
    .status(200)
    .header("content-type", "text/html")
    .body("Hello AWS Lambda HTTP request".into())
    .map_err(Box::new)?;
  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing::init_default_subscriber();
  run(service_fn(function_handler)).await
}
