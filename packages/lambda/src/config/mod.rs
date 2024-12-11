pub struct Config {
  pub cognito_origin: String,
  pub cognito_secret: String,
  pub cognito_client_id: String,
  pub login_endpoint: String,
  pub logout_endpoint: String,
  pub token_endpoint: String,
  pub local_origin: String,
}

impl Config {
  pub fn from_env() -> anyhow::Result<Self> {
    Ok(Self {
      cognito_origin: std::env::var("COGNITO_ORIGIN")?,
      cognito_secret: std::env::var("COGNITO_SECRET")?,
      cognito_client_id: std::env::var("COGNITO_CLIENT_ID")?,
      local_origin: std::env::var("LOCAL_ORIGIN")?,
      login_endpoint: "/oauth2/authorize".into(),
      logout_endpoint: "/logout".into(),
      token_endpoint: "/oauth2/token".into(),
    })
  }
}
