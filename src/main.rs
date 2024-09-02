use clap::Parser;
use rcli::Args;
use rcli::CmdExector;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    args.command.execute().await.unwrap();
}
