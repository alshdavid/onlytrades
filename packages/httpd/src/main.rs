mod handlers;
mod http1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  http1::http1_server("0.0.0.0:4200", |req, res| async move {
    handlers::not_found::handler(req, res).await
  })
  .await?;

  Ok(())
}
