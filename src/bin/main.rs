use loco_rs::cli;
use oic_fox_fuckery::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App>().await
}
