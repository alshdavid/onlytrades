use std::cell::OnceCell;
use std::collections::HashMap;
use std::sync::Arc;

use lambda_http::Request;
use lambda_http::RequestExt;

use crate::cognito::CognitoService;
use crate::config::Config;

pub mod auth_login_callback_get;
pub mod auth_login_get;
pub mod auth_logout_callback_get;
pub mod auth_logout_get;
pub mod auth_refresh_get;
pub mod auth_validate_get;
pub mod not_found_any;
pub mod ping_any;

pub struct Context {
  pub req: Arc<Request>,
  pub config: Config,
  pub query: QueryString,
  pub cognito_service: CognitoService,
}

pub struct QueryString(Arc<Request>, OnceCell<HashMap<String, String>>);

impl QueryString {
  pub fn new(req: Arc<Request>) -> Self {
    Self(req, Default::default())
  }

  pub fn all(&self) -> &HashMap<String, String> {
    self.1.get_or_init(move || {
      let qs = self.0.query_string_parameters();
      qs.iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    })
  }

  pub fn get(
    &self,
    key: &str,
  ) -> Option<&String> {
    self.all().get(key)
  }
}
