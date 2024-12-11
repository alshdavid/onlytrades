use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::http1::Bytes;
use crate::http1::HttpRequest;
use crate::http1::HttpResponse;
use crate::http1::HttpResult;

const CLIENT_PATH: Lazy<PathBuf> =
  Lazy::new(|| PathBuf::from(std::env::var("CLIENT_PATH").unwrap()));
const INDEX_PATH: Lazy<PathBuf> = Lazy::new(|| CLIENT_PATH.join("index.html"));

pub async fn handler(
  req: HttpRequest,
  mut res: HttpResponse,
) -> HttpResult {
  let req_path = req.uri().path()[1..].to_string();
  let file_path = CLIENT_PATH.join(req_path.clone());

  // Fallback Path
  if !file_path.exists() || (file_path.exists() && file_path.is_dir()) {
    return Ok(
      res
        .status(200)
        .header("Content-Type", "text/html")
        .body(Bytes::from(&fs::read(&*INDEX_PATH).unwrap()).into())?,
    );
  }

  if let Some(mime) = mime_guess::from_path(&file_path).first() {
    res = res.header("Content-Type", mime.to_string());
  }

  let Ok(contents) = fs::read(&file_path) else {
    return Ok(
      res
        .status(500)
        .body(Bytes::from("Unable to open file").into())?,
    );
  };

  Ok(res.status(200).body(Bytes::from(contents).into())?)
}
