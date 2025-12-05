use tokio::{io::BufStream, net::TcpListener};
use tracing::info;

mod req;

static PORT: &str = "8080";
static IP_EXPORT: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(format!("{IP_EXPORT}:{PORT}")).await?;

    info!("listening on: {}", listener.local_addr()?);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        let mut stream = BufStream::new(stream);

        // Spawn a new task
        tokio::spawn(async move {
            info!(?addr, "new connection");

            match req::parse_request(&mut stream).await {
                Ok(req) => info!(?req, "incoming request"),
                Err(e) => {
                    info!(?e, "failed to parse request");
                }
            }
        });
    }
}
