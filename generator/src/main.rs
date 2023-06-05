use anyhow::Result;

const _GENERATED_PATH: &str = "./generated";

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "[main] Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    Ok(())
}
