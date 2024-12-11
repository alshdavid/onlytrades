use std::collections::HashMap;

use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Request;
use lambda_http::Response;
use url::Url;

use crate::config::Config;

pub async fn handler(
  _event: Request,
  url: Url,
  config: &Config,
) -> Result<Response<Body>, Error> {
  let mut target = format!(
    "{}{}?response_type=code&client_id={}&redirect_uri={}/api/auth/login/callback",
    config.cognito_origin, config.login_endpoint, config.cognito_client_id, config.local_origin
  );

  let qs: HashMap<String, String> = url.query_pairs().into_owned().collect();
  if let Some(state) = qs.get("state") {
    target += &format!("&state={}", state)
  }

  Ok(
    Response::builder()
      .status(307)
      .header("Location", target)
      .body(vec![].into())
      .map_err(Box::new)?,
  )
}
