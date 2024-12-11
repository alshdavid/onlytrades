use serde::Deserialize;
use serde::Serialize;
use ureq;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenExchange {
  pub id_token: String,
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in: usize,
  pub token_type: String,
}

pub struct CognitoService {
  pub cognito_origin: String,
  pub token_endpoint: String,
  pub local_origin: String,
  pub cognito_client_id: String,
  pub cognito_secret: String,
}

impl CognitoService {
  pub fn token_exchange(
    &self,
    code: &str,
  ) -> anyhow::Result<TokenExchange> {
    let target = format!(
      "{}{}?scope=email/openid",
      self.cognito_origin, self.token_endpoint
    );
    let redirect_uri = format!("{}/api/auth/login/callback", self.local_origin);

    let mut form = Vec::<(&str, &str)>::new();
    form.push(("client_id", &self.cognito_client_id));
    form.push(("client_secret", &self.cognito_secret));
    form.push(("grant_type", "authorization_code"));
    form.push(("code", code));
    form.push(("redirect_uri", &redirect_uri));

    Ok(ureq::post(&target).send_form(&form)?.into_json()?)
  }
}
