use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Request;
use lambda_http::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PingAnyResponse {
  pub message: String,
}

pub async fn handler(_event: Request) -> Result<Response<Body>, Error> {
  let resp = serde_json::to_vec(&PingAnyResponse {
    message: "Hello World".into(),
  })?;

  Ok(
    Response::builder()
      .status(200)
      .header("Content-Type", "application/json")
      .body(resp.into())
      .map_err(Box::new)?,
  )
}
