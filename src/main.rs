use clap::Parser;
use rcli::Args;
use rcli::CmdExector;

fn main() {
    let args = Args::parse();
    args.command.execute().unwrap();
}
