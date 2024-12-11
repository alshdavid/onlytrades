use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Request;
use lambda_http::Response;

pub async fn handler(_event: Request) -> Result<Response<Body>, Error> {
  Ok(
    Response::builder()
      .status(200)
      .header("Content-Type", "application/json")
      .body("{\"message\":\"pong\"}".into())
      .map_err(Box::new)?,
  )
}
