use anyhow::Result;
use clipboard_listener::clipboard_lisen;
#[tokio::main]
async fn main() -> Result<()> {
    clipboard_lisen(|v| println!("{},{:?},{}", v.hash, v.tp, v.bytes.len()), 500);
    tokio::signal::ctrl_c().await?;
    Ok(())
}
