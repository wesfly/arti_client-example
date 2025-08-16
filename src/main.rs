use anyhow::Result;
use arti_client::{TorClient, TorClientConfig};
use futures::io::{AsyncReadExt, AsyncWriteExt};
use once_cell::sync::OnceCell;
use tokio;
use tor_rtcompat::PreferredRuntime;

static TOR_CLIENT: OnceCell<TorClient<PreferredRuntime>> = OnceCell::new();

pub fn get_tor_client() -> Result<TorClient<PreferredRuntime>> {
    let client = TOR_CLIENT.get_or_try_init(|| -> Result<TorClient<_>> {
        let config = TorClientConfig::default();

        #[cfg(debug_assertions)]
        eprintln!("creating unbootstrapped Tor client");

        Ok(TorClient::builder()
            .config(config)
            .create_unbootstrapped()?)
    })?;

    Ok(client.clone())
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    eprintln!("getting shared Tor client...");

    let tor_client = get_tor_client()?;
    // The BBC tor site
    let onion_address = "www.bbcweb3hytmzhn5d532owbu6oqadra5z3ar726vq5kgwwn6aucdccrad.onion:80";

    #[cfg(debug_assertions)]
    eprintln!("connecting to hidden service: {onion_address}");
    let mut stream = tor_client.connect(onion_address).await?;

    #[cfg(debug_assertions)]
    eprintln!("sending request...");
    let request = format!("GET / HTTP/1.1\r\nHost: {onion_address}\r\nConnection: close\r\n\r\n)");
    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;

    #[cfg(debug_assertions)]
    eprintln!("reading response...");
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}
