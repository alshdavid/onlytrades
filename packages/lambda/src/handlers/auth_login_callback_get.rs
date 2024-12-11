use lambda_http::Body;
use lambda_http::Error;
use lambda_http::Response;

use super::Context;

pub async fn handler(
  Context {
    query,
    cognito_service,
    ..
  }: Context
) -> Result<Response<Body>, Error> {
  let Some(code) = query.get("code") else {
    return Ok(
      Response::builder()
        .status(400)
        .body("".into())
        .map_err(Box::new)?,
    );
  };

  let token = cognito_service.token_exchange(code)?;
  let payload = token.id_token.split_terminator(".").collect::<Vec<&str>>()[1];
  let one_year = now_plus_days(365)?;
  Ok(
    Response::builder()
      .status(200)
      .header("Content-Type", "text/html")
      .header(
        "Set-Cookie",
        format!(
          "auth_refresh_token={}; SameSite=Strict; Path=/api/auth/refresh; HttpOnly; Expires={}",
          token.refresh_token, one_year
        ),
      )
      .header(
        "Set-Cookie",
        format!(
          "auth_id_token=${}; SameSite=Strict; Path=/api; HttpOnly; Expires={}",
          token.id_token, one_year
        ),
      )
      .body(
        format!(
          "<html><script>
            const payload = JSON.parse(atob('{}'))
            window.localStorage.setItem('onlytrades::auth', JSON.stringify(payload));
            window.location.assign('/')
          </script></html>",
          payload
        )
        .into(),
      )
      .map_err(Box::new)?,
  )
}

fn now_plus_days(days: u64) -> anyhow::Result<String> {
  let now = chrono::offset::Utc::now();
  let Some(now) = now.checked_add_days(chrono::Days::new(days)) else {
    anyhow::bail!("date error")
  };
  Ok(format!("{}", now.format("%+")))
}
